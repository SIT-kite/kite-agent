use super::{Request, RequestPayload, Response, ResponsePayload};
use crate::communication::AgentData;

impl Response {
    /// Respond normally.
    pub fn normal(payload: ResponsePayload) -> Self {
        // TODO
        let payload = bincode::serialize(&payload).unwrap();

        Self {
            ack: 0,
            code: 0,
            payload: payload,
        }
    }
    pub fn error(code: u16) -> Self {
        Self {
            ack: 0,
            code,
            payload: vec![],
        }
    }
    pub fn ack(mut self, ack: usize) -> Self {
        self.ack = ack;
        self
    }
}

async fn dispatch_command(seq: usize, request: RequestPayload, parameter: AgentData) -> Response {
    let response = match request {
        RequestPayload::AgentInfo(r) => r.process(parameter).await,
        RequestPayload::ElectricityBill(r) => r.process(parameter).await,
        RequestPayload::ActivityList(r) => r.process(parameter).await,
    };
    response.ack(seq)
}

pub async fn on_new_request(request: Request, data: AgentData) -> Response {
    let request_body = bincode::deserialize::<RequestPayload>(&request.payload);

    if let Ok(body) = request_body {
        dispatch_command(request.seq, body, data).await
    } else {
        Response::error(1).ack(request.seq)
    }
}
