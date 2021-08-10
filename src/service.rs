mod error;
pub mod report;
mod sc;

use serde::{Deserialize, Serialize};

pub use crate::net::auth::portal_login;
pub use error::{ActionError, ErrorResponse};

pub use report::AgentInfoRequest;
pub use sc::ActivityDetailRequest;
pub use sc::ActivityListRequest;

use crate::agent::SharedData;
use crate::parser::{Activity, ActivityDetail};
use report::AgentInfo;

/// Response payload
#[derive(Debug, Deserialize)]
pub enum RequestPayload {
    None,
    AgentInfo(AgentInfoRequest),
    ActivityList(ActivityListRequest),
    ActivityDetail(ActivityDetailRequest),
}

/// Response payload
#[derive(Debug, Serialize)]
pub enum ResponsePayload {
    None,
    Credential(AgentInfo),
    ActivityList(Vec<Activity>),
    ActivityDetail(Box<ActivityDetail>),
}

#[async_trait::async_trait]
pub trait DoRequest {
    async fn process(self, data: SharedData) -> ResponseResult;
}

/// Concat parameters to a url-formed string.
#[macro_export]
macro_rules! make_parameter {
    // Concatenate web form parameters to a string.
    ($($para: expr => $val: expr), *) => {{
        let mut url = String::new();
        $( url = url + $para + "=" + $val + "&"; )*

        url.clone()
    }}
}

// Result has two sides, Ok(ResponsePayload) and Err(ResponseError)
pub type ResponseResult = std::result::Result<ResponsePayload, ErrorResponse>;

impl RequestPayload {
    pub(crate) async fn dispatch(self, data: SharedData) -> ResponseResult {
        match self {
            RequestPayload::None => Ok(ResponsePayload::None),
            RequestPayload::AgentInfo(r) => r.process(data).await,
            RequestPayload::ActivityList(r) => r.process(data).await,
            RequestPayload::ActivityDetail(r) => r.process(data).await,
        }
    }
}
