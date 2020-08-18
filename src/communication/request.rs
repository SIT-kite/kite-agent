use super::Response;
use crate::communication::{AgentData, ResponsePayload};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AgentInfoRequest;

#[derive(Serialize)]
pub struct AgentInfo {
    pub name: String,
}

impl AgentInfoRequest {
    pub async fn process(self, parameter: AgentData) -> Response {
        Response::normal(ResponsePayload::Credential(AgentInfo {
            name: parameter.agent,
        }))
    }
}
