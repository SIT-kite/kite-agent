use super::{Request, RequestPayload, Response, ResponsePayload};
use crate::communication::{AgentData, Handle};

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

fn dispatch_command<D: Clone>(seq: usize, request: RequestPayload, parameter: AgentData<D>) -> Response {
    let response = match request {
        RequestPayload::AgentInfo(r) => r.process(parameter),
    };
    response.ack(seq)
}

pub fn on_new_request<D: Clone>(request: Request, data: AgentData<D>) -> Response {
    let request_body = bincode::deserialize::<RequestPayload>(&request.payload);

    if let Ok(body) = request_body {
        dispatch_command(request.seq, body, data)
    } else {
        Response::error(1).ack(request.seq)
    }
}
