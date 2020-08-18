use super::{Agent, AgentBuilder, AgentData, Request, Response};
use crate::net::SessionStorage;
use futures::Future;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

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
    pub fn host(mut self, addr: &str) -> Self {
        self.host_addr = Some(addr.to_string());
        self
    }

    /// Set callback function which will be called when packet comes.
    pub fn set_callback(mut self, callback_fn: MessageCallbackFn<O>, parameter: SessionStorage) -> Self {
        self.message_callback = Some(MessageCallback {
            function: callback_fn,
            parameter: AgentData {
                agent: String::new(),
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
        content: Vec<u8>,
        mut socket_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<O>>,
    ) {
        let request = bincode::deserialize(&content);
        if let Ok(req) = request {
            // Get callback function pointer and parameter.
            let request_callback = on_message.function;
            let callback_parameter = on_message.parameter.clone();

            // TODO: Return result instead of doing nothing.
            // If callback functions successfully, serialize the response and send back to host.
            let response = request_callback(req, callback_parameter).await;
            let response_content = bincode::serialize(&response);
            if let Ok(response_content) = response_content {
                socket_tx.send(Message::Binary(response_content)).await;
            }
        }
        // TODO: Send error code `unknown`.
    }

    /// Unpack WebSocket message, match types and respond correctly.
    async fn process_message(
        message: Message,
        mut message_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<O>>,
    ) {
        // Resolve request message, and response.
        // For Ping, Pong, Close message, we can send response immediately, while for binary we need
        // to decode and usually do further operation then.
        match message {
            Message::Binary(content) => {
                // Spawn new thread to execute the function because it usually costs a lot of time.
                actix_rt::spawn(async move {
                    Self::dispatch_message(content, message_tx, on_message.clone()).await
                });
            }
            Message::Ping(_) => {
                // Pong will be responded automatically by the framework.
                ()
            }
            Message::Pong(_) => {
                // Do nothing if Pong received
                ()
            }
            _ => {
                // When Message::Close or Message::Text (which unexpected for us) received,
                // close connection.
                message_tx.send(Message::Close(None)).await;
            }
        }
    }

    /// Receiver loop, accept commands and requests from the host.
    async fn receiver_loop<T>(
        mut socket_rx: T,
        message_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<O>>,
    ) where
        T: StreamExt + std::marker::Unpin,
        T::Item: Into<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
    {
        while let Some(r) = socket_rx.next().await {
            match r.into() {
                Ok(message) => {
                    Self::process_message(message, message_tx.clone(), on_message.clone()).await
                }
                Err(_) => {}
            }
        }
    }

    /// Send response to host.
    async fn sender_loop<T, Item>(mut socket_tx: T, mut message_rx: mpsc::Receiver<Message>)
    where
        T: SinkExt<Item> + std::marker::Unpin,
        Item: From<Message>,
    {
        while let Some(response) = message_rx.recv().await {
            socket_tx.send(response.into()).await;
        }
    }

    /// Connect to host and start necessary event loop for communication over WebSocket.
    pub async fn start(&mut self) {
        let (socket, _) = tokio_tungstenite::connect_async(&self.host_addr).await.unwrap();
        let (write, read) = socket.split();
        let (tx, rx) = mpsc::channel::<Message>(128);

        // Spawn receiver loop.
        tokio::spawn(Self::receiver_loop(read, tx, self.message_callback.clone()));
        // Spawn sender loop.
        tokio::spawn(Self::sender_loop(write, rx));
    }
}
