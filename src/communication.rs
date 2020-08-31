mod agent;
mod process;

use crate::net::SessionStorage;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::ResponseResult;
pub use agent::{MessageCallback, MessageCallbackFn};
use bytes::{Buf, BytesMut};
pub use process::on_new_request;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::error::Result;

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
    async fn read_header<T: AsyncRead + AsyncReadExt + Unpin>(stream: &mut T) -> Result<Self> {
        // Default request header is 8 bytes.
        let mut buffer = BufReader::with_capacity(12, stream);
        let mut request = Request::default();

        // Read the fields
        request.seq = buffer.read_u64().await?;
        request.size = buffer.read_u32().await?;

        Ok(request)
    }

    pub async fn from_stream<T: AsyncReadExt + Unpin>(
        stream: &mut T,
        buffer: &mut BytesMut,
    ) -> Result<Request> {
        let mut request = Self::read_header(stream).await?;

        if request.size == 0 {
            return Ok(request);
        }
        if buffer.capacity() < request.size as usize {
            buffer.resize(request.size as usize, 0u8);
        }
        // Read request body
        let mut p = 0usize; // read len
        while p < request.size as usize {
            let mut read_currently = request.size as usize - p;
            if read_currently > 2048 {
                read_currently = 2048usize;
            }
            p += stream.read_exact(&mut buffer[p..(p + read_currently)]).await?;
        }
        request.payload = buffer.to_vec();
        Ok(request)
    }
}
