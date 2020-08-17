use super::Handle;
use super::Response;
use crate::communication::{AgentData, ResponsePayload};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AgentInfoRequest;

#[derive(Serialize)]
pub struct AgentInfo {
    pub name: String,
}

impl<D: Clone> Handle<D> for AgentInfoRequest {
    fn process(self, parameter: AgentData<D>) -> Response {
        Response::normal(ResponsePayload::Credential(AgentInfo {
            name: parameter.agent,
        }))
    }
}

pub struct ElectricityBillRequest {
    pub room: String,
}

// impl<D: Clone> Handle<D> for ElectricityBillRequest {
//     fn process(self, parameter: AgentData<D>) -> Response {
//         ElectricityBill
//     }
// }
