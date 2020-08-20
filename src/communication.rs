mod agent;
mod error;
mod process;
mod request;

use crate::net::SessionStorage;
use futures::Future;
use request::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

use crate::parser::Activity;
use crate::parser::ElectricityBill;
use crate::service::ActivityListRequest;
use crate::service::ElectricityBillRequest;

/// Response payload
#[derive(Deserialize)]
pub enum RequestPayload {
    AgentInfo(AgentInfoRequest),
    ElectricityBill(ElectricityBillRequest),
    ActivityList(ActivityListRequest),
}

/// Response payload
#[derive(Serialize)]
pub enum ResponsePayload {
    Credential(AgentInfo),
    ElectricityBill(ElectricityBill),
    ActivityList(Vec<Activity>),
}

#[derive(Clone)]
pub struct AgentData {
    pub agent: String,
    pub local_addr: String,

    pub parameter: SessionStorage,
}
