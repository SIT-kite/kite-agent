use super::{Request, Response};
use crate::communication::AgentData;
use crate::service::{ActionError, ErrorResponse, RequestPayload, ResponsePayload};

impl Response {
    /// Respond normally.
    pub fn normal(payload: ResponsePayload) -> Self {
        let payload = bincode::serialize(&payload).unwrap();
        Self::raw(0, payload)
    }
    pub fn error(code: u16, msg: String) -> Self {
        Self::raw(code, Vec::from(msg))
    }

    #[inline]
    fn raw(code: u16, payload: Vec<u8>) -> Self {
        Self {
            ack: 0,
            size: payload.len() as u32,
            code,
            payload,
        }
    }
    pub fn ack(mut self, ack: u64) -> Self {
        self.ack = ack;
        self
    }
}

async fn dispatch_command(seq: u64, request: RequestPayload, parameter: AgentData) -> Response {
    let response: Response = match request {
        RequestPayload::AgentInfo(r) => r.process(parameter).await,
        RequestPayload::ElectricityBill(r) => r.process(parameter).await,
        RequestPayload::ActivityList(r) => r.process(parameter).await,
        RequestPayload::ScoreList(r) => r.process(parameter).await,
    }
    .into();

    response.ack(seq)
}

pub async fn on_new_request(request: Request, data: AgentData) -> Response {
    let request_body = bincode::deserialize::<RequestPayload>(&request.payload);

    if let Ok(body) = request_body {
        dispatch_command(request.seq, body, data).await
    } else {
        let e: ErrorResponse = ActionError::BadRequest.into();
        Response::error(e.code, e.msg).ack(request.seq)
    }
}
