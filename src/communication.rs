mod agent;
mod process;

use crate::net::SessionStorage;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::ResponseResult;
pub use process::on_new_request;
use tokio::io::{AsyncReadExt, BufReader};

use crate::error::Result;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::broadcast;

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

/// Agent instance builder
pub struct AgentBuilder<O>
where
    O: Future<Output = Response> + Send + 'static,
{
    /// Local agent name
    name: String,
    /// Host url, a string like "wss://example.com/ws/"
    host_addr: Option<String>,
    /// Callback structure, with callback function point and parameter.
    message_callback: Option<MessageCallback<O>>,
}

struct HaltChannel {
    sender: broadcast::Sender<()>,
    receiver: broadcast::Receiver<()>,
}

/// Agent node in campus side.
pub struct Agent<O>
where
    O: Future<Output = Response> + Send + 'static,
{
    /// Local agent name
    name: String,
    /// Host url, a string like "wss://example.com/ws/"
    host_addr: String,
    /// Callback structure, with callback function point and parameter.
    message_callback: Arc<MessageCallback<O>>,
    /// Halt channel
    halt: Option<HaltChannel>,
}

/// Host request
#[derive(Default, Deserialize)]
pub struct Request {
    /// Request sequence
    pub seq: u64,
    /// Packet size
    pub size: u32,
    /// Payload
    pub payload: Vec<u8>,
}

/// Agent response
#[derive(Default, Debug, Serialize)]
pub struct Response {
    /// Response sequence
    pub ack: u64,
    /// Packet size
    pub size: u32,
    /// Status code
    pub code: u16,
    /// Payload
    pub payload: Vec<u8>,
}

#[derive(Clone)]
pub struct AgentData {
    pub agent: String,
    pub local_addr: String,

    pub parameter: SessionStorage,
}

impl From<ResponseResult> for Response {
    fn from(result: ResponseResult) -> Self {
        match result {
            Ok(payload) => Response::normal(payload),
            Err(e) => Response::error(e.code, e.msg),
        }
    }
}

impl Request {
    async fn read_header(buffer: &mut BufReader<OwnedReadHalf>) -> Result<Self> {
        let mut request = Request::default();

        // Read the fields
        request.seq = buffer.read_u64().await?;
        request.size = buffer.read_u32().await?;

        Ok(request)
    }

    pub async fn from_stream(buffer: &mut BufReader<OwnedReadHalf>) -> Result<Request> {
        let mut request = Self::read_header(buffer).await?;
        if request.size == 0 {
            return Ok(request);
        }
        request.payload = vec![0u8; request.size as usize];

        // Read request body
        let mut p = 0usize; // read len
        while p < request.size as usize {
            let mut read_currently = request.size as usize - p;
            if read_currently > 2048 {
                read_currently = 2048usize;
            }
            p += buffer
                .read_exact(&mut request.payload[p..(p + read_currently)])
                .await?;
        }
        Ok(request)
    }
}
