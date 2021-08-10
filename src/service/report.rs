use crate::service::{ResponsePayload, ResponseResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AgentInfoRequest;

#[derive(Debug, Serialize)]
pub struct AgentInfo {
    pub name: String,
}
