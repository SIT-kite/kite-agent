use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_bincode::AsyncBincodeStream;
use serde::{Deserialize, Serialize};
use tokio_tower::multiplex;
use tokio_tower::multiplex::Server;
use tower::Service;

use crate::error::{AgentError, Result};
use crate::service::{RequestPayload, ResponsePayload, ResponseResult};
use crate::SessionStorage;

#[derive(Debug, Deserialize)]
struct RequestFrame {
    payload: RequestPayload,
}

#[derive(Debug, Serialize)]
struct ResponseFrame {
    payload: ResponseResult,
}

impl Default for ResponseFrame {
    fn default() -> Self {
        ResponseFrame {
            payload: Ok(ResponsePayload::None),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedData {
    pub node: String,
    pub client: reqwest::Client,
    pub session_store: SessionStorage,
}

#[derive(Debug, Default)]
// only pub because we use it to figure out the error type for ViewError
struct Tagger(slab::Slab<()>);

impl<Request: core::fmt::Debug, Response: core::fmt::Debug>
    multiplex::TagStore<Tagged<Request>, Tagged<Response>> for Tagger
{
    type Tag = u32;

    fn assign_tag(mut self: Pin<&mut Self>, r: &mut Tagged<Request>) -> Self::Tag {
        r.tag = self.0.insert(()) as u32;
        r.tag
    }
    fn finish_tag(mut self: Pin<&mut Self>, r: &Tagged<Response>) -> Self::Tag {
        self.0.remove(r.tag as usize);
        r.tag
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Tagged<T>
where
    T: core::fmt::Debug,
{
    pub v: T,
    pub tag: u32,
}

impl<T: core::fmt::Debug> From<T> for Tagged<T> {
    fn from(t: T) -> Self {
        Tagged { tag: 0, v: t }
    }
}

#[derive(Debug, Clone)]
struct KiteService {
    shared_data: SharedData,
}

impl Service<Tagged<RequestFrame>> for KiteService {
    type Response = Tagged<ResponseFrame>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Tagged<RequestFrame>) -> Self::Future {
        // Note: Maybe improve performance
        let data = self.shared_data.clone();

        let f = async move {
            let tag = req.tag;
            println!("Received frame: {:?}, tag = {}", &req.v, tag);

            let request_frame = req.v;
            let response_frame = ResponseFrame {
                payload: request_frame.payload.dispatch(data).await,
            };
            let mut response = Tagged::<ResponseFrame>::from(response_frame);

            response.tag = tag;
            Ok(response)
        };

        Box::pin(f)
    }
}

pub async fn run(server_address: String, shared_data: SharedData) -> Result<()> {
    println!("Connecting to server: {}", server_address);
    // Create a socket and connect to server.
    let socket = tokio::net::TcpStream::connect(server_address)
        .await
        .map_err(|_| AgentError::ConnectionFailure)?;

    println!("Connected.");

    Server::new(
        AsyncBincodeStream::from(socket).for_async(),
        KiteService { shared_data },
    )
    .await
    .map_err(|e| AgentError::Service(e.to_string()))?;

    println!("Disconnected.");
    Ok(())
}
