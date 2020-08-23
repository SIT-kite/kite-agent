mod agent;
mod process;

use crate::net::SessionStorage;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::ResponseResult;
pub use agent::{MessageCallback, MessageCallbackFn};
pub use process::on_new_request;

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
#[derive(Deserialize)]
pub struct Request {
    /// Request sequence
    pub seq: usize,
    /// Request type
    // pub code: u16,
    /// Payload
    pub payload: Vec<u8>,
}

/// Agent response
#[derive(Debug, Serialize)]
pub struct Response {
    /// Response sequence
    pub ack: usize,
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
