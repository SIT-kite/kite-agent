use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

#[derive(Deserialize)]
pub struct Request;

#[derive(Serialize)]
pub struct Response;

type OnMessageCallback<Data> = fn(Request, Data) -> crate::error::Result<Response>;

struct MessageCallback<Data>
where
    Data: Clone + Send + Sync + 'static,
{
    pub function: OnMessageCallback<Data>,
    pub parameter: Data,
}

pub struct AgentBuilder<D: Clone + Send + Sync + 'static> {
    name: String,
    host_addr: Option<String>,
    request_queue: Arc<Vec<(Request, mpsc::Sender<i32>)>>,
    send_socket: Option<mpsc::Sender<Message>>,
    message_callback: Option<MessageCallback<D>>,
}

impl<D: Clone + Send + Sync + 'static> AgentBuilder<D> {
    /// Create a new agent instance.
    pub fn new(name: String) -> Self {
        Self {
            name,
            host_addr: None,
            request_queue: Arc::new(vec![]),
            send_socket: None,
            message_callback: None,
        }
    }

    /// Set host address
    pub fn host(mut self, addr: &str) -> Self {
        self.host_addr = Some(addr.to_string());
        self
    }

    /// Set callback function which will be called when packet comes.
    pub fn set_callback(mut self, callback_fn: OnMessageCallback<D>, parameter: D) -> Self {
        self.message_callback = Some(MessageCallback {
            function: callback_fn,
            parameter,
        });
        self
    }

    pub fn finish(self) -> Agent<D> {
        Agent {
            name: self.name,
            host_addr: self.host_addr.expect("Host address is needed."),
            message_callback: Arc::new(
                self.message_callback.expect("You should set callback function."),
            ),
            request_queue: Arc::new(vec![]),
            send_socket: None,
        }
    }
}

pub struct Agent<D>
where
    D: Clone + Send + Sync + 'static,
{
    name: String,
    host_addr: String,
    request_queue: Arc<Vec<(Request, mpsc::Sender<i32>)>>,
    send_socket: Option<mpsc::Sender<Message>>,
    message_callback: Arc<MessageCallback<D>>,
}

impl<D> Agent<D>
where
    D: Clone + Send + Sync + 'static,
{
    async fn dispatch_message(
        content: Vec<u8>,
        mut socket_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<D>>,
    ) {
        let request = bincode::deserialize(&content);
        if let Ok(req) = request {
            let request_callback = on_message.function;
            let callback_parameter = on_message.parameter.clone();

            if let Ok(response) = request_callback(req, callback_parameter) {
                // Pack
                let response_content = bincode::serialize(&response);
                if let Ok(response_content) = response_content {
                    socket_tx.send(Message::Binary(response_content)).await;
                }
            }
        }
    }
    async fn process_message(
        message: Message,
        mut socket_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<D>>,
    ) {
        if let Message::Binary(content) = message {
            actix_rt::spawn(Self::dispatch_message(content, socket_tx, on_message.clone()));
        } else if let Message::Ping(content) = message {
            socket_tx.send(Message::Pong(content)).await;
        } else if let Message::Pong(_) = message {
            ()
        } else {
            // When Message::Close or Message::Text (which unexpected for us) received,
            // close connection.
            socket_tx.send(Message::Close(None)).await;
        }
    }

    async fn receiver_loop<T>(
        mut rx: T,
        socket_tx: mpsc::Sender<Message>,
        on_message: Arc<MessageCallback<D>>,
    ) where
        T: StreamExt + std::marker::Unpin,
        T::Item: Into<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
    {
        while let Some(r) = rx.next().await {
            match r.into() {
                Ok(message) => {
                    Self::process_message(message, socket_tx.clone(), on_message.clone()).await
                }
                Err(e) => {}
            }
        }
    }

    async fn sender_loop<T, Item>(mut socket_tx: T, mut message_rx: mpsc::Receiver<Message>)
    where
        T: SinkExt<Item> + std::marker::Unpin,
        Item: From<Message>,
    {
        while let Some(response) = message_rx.recv().await {
            socket_tx.send(response.into()).await;
        }
    }

    pub async fn start(&mut self) {
        let (mut socket, _) = tokio_tungstenite::connect_async(&self.host_addr).await.unwrap();
        let (mut write, mut read) = socket.split();
        let (tx, mut rx) = mpsc::channel::<Message>(128);

        // Spawn receiver loop.
        tokio::spawn(Self::receiver_loop(read, tx, self.message_callback.clone()));
        // Spawn sender loop.
        tokio::spawn(Self::sender_loop(write, rx));
    }
}
