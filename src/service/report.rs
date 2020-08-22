use crate::communication::AgentData;
use crate::service::{ResponsePayload, ResponseResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AgentInfoRequest;

#[derive(Serialize)]
pub struct AgentInfo {
    pub name: String,
}

impl AgentInfoRequest {
    pub async fn process(self, parameter: AgentData) -> ResponseResult {
        Ok(ResponsePayload::Credential(AgentInfo {
            name: parameter.agent,
        }))
    }
}
