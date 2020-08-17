mod agent;
mod error;
mod process;
mod request;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub use agent::{MessageCallback, MessageCallbackFn};
pub use process::on_new_request;

/// Agent instance builder
pub struct AgentBuilder<T: Clone + Send + Sync + 'static> {
    /// Local agent name
    name: String,
    /// Host url, a string like "wss://example.com/ws/"
    host_addr: Option<String>,
    /// Callback structure, with callback function point and parameter.
    message_callback: Option<MessageCallback<T>>,
}

/// Agent node in campus side.
pub struct Agent<D>
where
    D: Clone + Send + Sync + 'static,
{
    /// Local agent name
    name: String,
    /// Host url, a string like "wss://example.com/ws/"
    host_addr: String,
    /// Callback structure, with callback function point and parameter.
    message_callback: Arc<MessageCallback<D>>,
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
#[derive(Serialize)]
pub struct Response {
    /// Response sequence
    pub ack: usize,
    /// Status code
    pub code: u16,
    /// Payload
    pub payload: Vec<u8>,
}

use request::*;

/// Response payload
#[derive(Deserialize)]
pub enum RequestPayload {
    AgentInfo(AgentInfoRequest),
}

/// Response payload
#[derive(Serialize)]
pub enum ResponsePayload {
    Credential(AgentInfo),
}

#[derive(Clone)]
pub struct AgentData<D: Clone> {
    pub agent: String,
    pub local_addr: String,

    pub parameter: D,
}

impl<D: Clone> AgentData<D> {
    pub fn new(parameter: D) -> Self {
        Self {
            agent: String::new(),
            local_addr: String::new(),
            parameter,
        }
    }
}

pub trait Handle<D: Clone> {
    fn process(self, parameter: AgentData<D>) -> Response;
}
