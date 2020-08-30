use super::{Agent, AgentBuilder, AgentData, Request, Response};
use crate::error::Result;
use crate::net::SessionStorage;
use bytes::BytesMut;
use futures::Future;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;

/// Message callback function
pub type MessageCallbackFn<O> = fn(Request, AgentData) -> O;

/// Message callback function and parameter
pub struct MessageCallback<O>
where
    O: Future<Output = Response> + Send + 'static,
{
    pub function: MessageCallbackFn<O>,
    pub parameter: AgentData,
}

impl<O> AgentBuilder<O>
where
    O: Future<Output = Response> + Send + 'static,
{
    /// Create a new agent instance.
    pub fn new(name: String) -> Self {
        Self {
            name,
            host_addr: None,
            message_callback: None,
        }
    }

    /// Set host address
    pub fn host(mut self, addr: String) -> Self {
        self.host_addr = Some(addr);
        self
    }

    /// Set callback function which will be called when packet comes.
    pub fn set_callback(mut self, callback_fn: MessageCallbackFn<O>, parameter: SessionStorage) -> Self {
        self.message_callback = Some(MessageCallback {
            function: callback_fn,
            parameter: AgentData {
                agent: self.name.clone(),
                local_addr: String::new(),
                parameter,
            },
        });
        self
    }

    /// Build a valid Agent structure. `panic` if host or callback function is not set.
    pub fn build(self) -> Agent<O> {
        let message_callback = self.message_callback.expect("You should set callback function.");

        Agent {
            name: self.name.clone(),
            host_addr: self.host_addr.expect("Host address is needed."),
            message_callback: Arc::new(message_callback),
        }
    }
}

impl<O> Agent<O>
where
    O: Future<Output = Response> + Send + 'static,
{
    /// Unpack binary request payload, do the command, then pack and send response to host.
    async fn dispatch_message(
        request: Request,
        mut socket_tx: mpsc::Sender<Response>,
        on_message: Arc<MessageCallback<O>>,
    ) -> Result<()> {
        // Get callback function pointer and parameter.
        let request_callback = on_message.function;
        let callback_parameter = on_message.parameter.clone();

        // If callback functions successfully, serialize the response and send back to host.
        let response = request_callback(request, callback_parameter).await;
        socket_tx.send(response).await?;

        Ok(())
    }

    /// Unpack WebSocket message, match types and respond correctly.
    async fn process_message(
        request: Request,
        mut response_tx: mpsc::Sender<Response>,
        on_message: Arc<MessageCallback<O>>,
    ) -> Result<()> {
        // Resolve request message, and response.
        // For Ping, we can send response immediately, while for binary we need to decode
        // and usually do further operation then.
        if request.seq == 0 || request.payload.is_empty() {
            response_tx.send(Response::default()).await?;
            return Ok(());
        }
        // Spawn new thread to execute the function because it usually costs a lot of time.
        tokio::spawn(
            async move { Self::dispatch_message(request, response_tx, on_message.clone()).await },
        );
        Ok(())
    }

    /// Receiver loop, accept commands and requests from the host.
    async fn receiver_loop(
        mut socket_rx: OwnedReadHalf,
        message_tx: mpsc::Sender<Response>,
        on_message: Arc<MessageCallback<O>>,
    ) {
        let mut buffer = BytesMut::with_capacity(1024 * 1024); // 1M receive buffer by default.

        while let Ok(r) = Request::from_stream(&mut socket_rx, &mut buffer).await {
            Self::process_message(r, message_tx.clone(), on_message.clone()).await;
        }
    }

    /// Send response to host.
    async fn sender_loop(mut socket_tx: OwnedWriteHalf, mut message_rx: mpsc::Receiver<Response>) {
        while let Some(response) = message_rx.recv().await {
            socket_tx.write_u64(response.ack).await;
            socket_tx.write_u16(response.code).await;
            socket_tx.write_all(&response.payload).await;
        }
    }

    /// Connect to host and start necessary event loop for communication over WebSocket.
    pub async fn start(&mut self) -> Result<()> {
        let s = tokio::net::TcpStream::connect(&self.host_addr).await?;
        let (read_half, write_half) = s.into_split();
        let (tx, rx) = mpsc::channel(128);

        // Spawn receiver loop.
        tokio::spawn(Self::receiver_loop(read_half, tx, self.message_callback.clone()));
        // Spawn sender loop.
        tokio::spawn(Self::sender_loop(write_half, rx));
        Ok(())
    }
}
