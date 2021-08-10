use crate::agent::SharedData;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AgentInfoRequest;

#[derive(Debug, Serialize)]
pub struct AgentInfo {
    pub name: String,
}

#[async_trait::async_trait]
impl DoRequest for AgentInfoRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let agent_info = AgentInfo { name: data.node };
        Ok(ResponsePayload::Credential(agent_info))
    }
}
