use serde::{Deserialize, Serialize};

use crate::agent::SharedData;
pub use crate::net::auth::portal_login;
use crate::parser::{Activity, ActivityDetail, Class, Course, Major, Profile, Score, ScScoreItem};
pub use edu::{
    ClassRequest, CourseRequest, MajorRequest, ProfileRequest, ScoreRequest, TimeTableRequest,
};
pub use error::{ActionError, ErrorResponse};
use report::AgentInfo;
pub use report::AgentInfoRequest;
pub use sc::ActivityDetailRequest;
pub use sc::ActivityListRequest;
use crate::service::sc::ScScoreItemRequest;

mod edu;
mod error;
pub mod report;
mod sc;

/// Response payload
#[derive(Debug, Deserialize)]
pub enum RequestPayload {
    None,
    Ping(String),
    AgentInfo(AgentInfoRequest),
    ActivityList(ActivityListRequest),
    ActivityDetail(ActivityDetailRequest),
    ScoreDetail(ScScoreItemRequest),
    MajorList(MajorRequest),
    // ClassList(ClassRequest),
    // CourseList(CourseRequest),
    // Profile(ProfileRequest),
    TimeTable(TimeTableRequest),
    Score(ScoreRequest),
}

/// Response payload
#[derive(Debug, Serialize)]
pub enum ResponsePayload {
    None,
    Pong(String),
    Credential(AgentInfo),
    ActivityList(Vec<Activity>),
    ActivityDetail(Box<ActivityDetail>),
    ScoreDetail(Vec<ScScoreItem>),
    MajorList(Vec<Major>),
    // ClassList(Vec<Class>),
    // CourseList(Vec<Course>),
    // Profile(Profile),
    TimeTable(Vec<Course>),
    Score(Vec<Score>),
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
            RequestPayload::Ping(r) => Ok(ResponsePayload::Pong(r)),
            RequestPayload::AgentInfo(r) => r.process(data).await,
            RequestPayload::ActivityList(r) => r.process(data).await,
            RequestPayload::ActivityDetail(r) => r.process(data).await,
            RequestPayload::ScoreDetail(r) => r.process(data).await?,
            RequestPayload::MajorList(r) => r.process(data).await,
            // RequestPayload::ClassList(r) => r.process(data).await,
            // RequestPayload::CourseList(r) => r.process(data).await,
            // RequestPayload::Profile(r) => r.process(data).await,
            RequestPayload::TimeTable(r) => r.process(data).await,
            RequestPayload::Score(r) => r.process(data).await,
        }
    }
}
