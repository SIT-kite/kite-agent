use crate::service::{ResponsePayload, ResponseResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AgentInfoRequest;

#[derive(Serialize)]
pub struct AgentInfo {
    pub name: String,
}
