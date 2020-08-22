mod bill;
mod course;
mod error;
pub mod report;
mod sc;

use serde::{Deserialize, Serialize};

pub use crate::net::auth::portal_login;
pub use error::{ActionError, ErrorResponse};

use crate::communication::Response;
pub use bill::ElectricityBillRequest;
pub use course::CourseScoreRequest;
pub use report::AgentInfoRequest;
pub use sc::ActivityListRequest;

use crate::parser::Activity;
use crate::parser::CourseScore;
use crate::parser::ElectricityBill;
use report::AgentInfo;

/// Response payload
#[derive(Deserialize)]
pub enum RequestPayload {
    AgentInfo(AgentInfoRequest),
    ElectricityBill(ElectricityBillRequest),
    ActivityList(ActivityListRequest),
    ScoreList(CourseScoreRequest),
}

/// Response payload
#[derive(Serialize)]
pub enum ResponsePayload {
    Credential(AgentInfo),
    ElectricityBill(ElectricityBill),
    ActivityList(Vec<Activity>),
    ScoreList(Vec<CourseScore>),
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
