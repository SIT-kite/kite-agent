use super::{Agent, AgentBuilder, AgentData, Request, Response};
use super::{MessageCallback, MessageCallbackFn};
use crate::communication::HaltChannel;
use crate::error::Result;
use crate::net::SessionStorage;
use futures::Future;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::{broadcast, mpsc};
use tokio::time::Duration;

impl Clone for HaltChannel {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
        }
    }
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
            halt: None,
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
        socket_rx: OwnedReadHalf,
        message_tx: mpsc::Sender<Response>,
        on_message: Arc<MessageCallback<O>>,
        mut halt: HaltChannel,
    ) {
        // 1M receive buffer by default.
        let mut buffer = BufReader::with_capacity(1024 * 1024, socket_rx);

        loop {
            tokio::select! {
                // Read the request packet from the host
                result = Request::from_stream(&mut buffer) => {
                    match result {
                        Ok(request) => {
                            Self::process_message(request, message_tx.clone(), on_message.clone()).await;
                        }
                        Err(_) => {
                            halt.sender.send(());
                            break;
                        }
                    }
                }
                // Got the halt message, break the loop and return.
                _ = halt.receiver.recv() => {
                    break;
                }
            }
        }
        println!("Receiver loop exited.");
    }

    /// Send response to host.
    async fn sender_loop(
        socket_tx: OwnedWriteHalf,
        mut message_rx: mpsc::Receiver<Response>,
        mut halt: HaltChannel,
    ) {
        let mut buffer = BufWriter::new(socket_tx);

        loop {
            tokio::select! {
                // Get a new response and send back to the host.
                Some(response) = message_rx.recv() => {
                    buffer.write_u64(response.ack).await;
                    buffer.write_u32(response.size).await;
                    buffer.write_u16(response.code).await;
                    if !response.payload.is_empty() {
                        buffer.write_all(&response.payload).await;
                    }
                    buffer.flush().await;
                }
                // Exit message received
                 _ = halt.receiver.recv() => {
                    break;
                 }
            };
        }
        println!("Sender loop exited.");
    }

    /// Connect to host and start necessary event loop for communication over WebSocket.
    pub async fn start(&mut self) -> Result<()> {
        let s = tokio::net::TcpStream::connect(&self.host_addr).await?;
        let (read_half, write_half) = s.into_split();

        let (tx, rx) = mpsc::channel(128);
        let (halt_tx, halt_rx) = broadcast::channel(1);

        // Spawn receiver loop.
        tokio::spawn(Self::receiver_loop(
            read_half,
            tx,
            self.message_callback.clone(),
            HaltChannel {
                sender: halt_tx.clone(),
                receiver: halt_tx.subscribe(),
            },
        ));
        // Spawn sender loop.
        tokio::spawn(Self::sender_loop(
            write_half,
            rx,
            HaltChannel {
                sender: halt_tx.clone(),
                receiver: halt_tx.subscribe(),
            },
        ));

        self.halt = Some(HaltChannel {
            sender: halt_tx.clone(),
            receiver: halt_rx,
        });

        Ok(())
    }

    pub fn available(&self) -> bool {
        self.halt.is_some()
    }

    pub async fn join(&mut self) {
        if let Some(mut channel) = self.halt.clone() {
            let mut rx = channel.receiver;

            rx.recv().await;
            tokio::time::delay_for(Duration::from_secs(1)).await;
            self.halt = None;

            return;
        }
    }

    /// Halt the agent client
    pub fn stop(&mut self) {
        if let Some(halt_channel) = self.halt.take() {
            halt_channel.sender.send(());
        }
    }
}
