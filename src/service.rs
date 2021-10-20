use serde::{Deserialize, Serialize};

use auth::{PortalAuthRequest, PortalAuthResponse};
pub use edu::{
    ClassRequest, CourseRequest, MajorRequest, ProfileRequest, ScoreDetailRequest, ScoreRequest,
    TimeTableRequest,
};
pub use error::{ActionError, ErrorResponse};
pub use library::{BookHoldingRequest, SearchLibraryRequest, SearchWay, SortOrder, SortWay};
use report::AgentInfo;
pub use report::AgentInfoRequest;
pub use sc::{ActivityDetailRequest, ActivityListRequest, ScActivityRequest, ScScoreItemRequest};

use crate::agent::SharedData;
pub use crate::net::auth::portal_login;
use crate::parser::{Activity, ActivityDetail, Course, HoldingPreviews, Major, ScActivityItem, ScScoreItem, Score, ScoreDetail, SearchLibraryResult, ExpensePage};
use crate::service::expense::ExpenseRequest;

mod auth;
mod edu;
mod error;
mod library;
pub mod report;
mod sc;
mod expense;

/// Response payload
#[derive(Debug, Deserialize)]
pub enum RequestPayload {
    None,
    Ping(String),
    AgentInfo(AgentInfoRequest),
    PortalAuth(PortalAuthRequest),
    ActivityList(ActivityListRequest),
    ActivityDetail(ActivityDetailRequest),
    ScMyScore(ScScoreItemRequest),
    ScMyActivity(ScActivityRequest),
    MajorList(MajorRequest),
    // ClassList(ClassRequest),
    // CourseList(CourseRequest),
    // Profile(ProfileRequest),
    TimeTable(TimeTableRequest),
    Score(ScoreRequest),
    ScoreDetail(ScoreDetailRequest),
    SearchLibrary(SearchLibraryRequest),
    BookHoldingInfo(BookHoldingRequest),
    CardExpense(ExpenseRequest),
}

/// Response payload
#[derive(Debug, Serialize)]
pub enum ResponsePayload {
    None,
    Pong(String),
    Credential(AgentInfo),
    PortalAuth(PortalAuthResponse),
    ActivityList(Vec<Activity>),
    ActivityDetail(Box<ActivityDetail>),
    ScMyScore(Vec<ScScoreItem>),
    ScMyActivity(Vec<ScActivityItem>),
    MajorList(Vec<Major>),
    // ClassList(Vec<Class>),
    // CourseList(Vec<Course>),
    // Profile(Profile),
    TimeTable(Vec<Course>),
    Score(Vec<Score>),
    ScoreDetail(Vec<ScoreDetail>),
    SearchLibrary(SearchLibraryResult),
    BookHoldingInfo(HoldingPreviews),
    CardExpense(ExpensePage),
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
            RequestPayload::PortalAuth(r) => r.process(data).await,
            RequestPayload::ActivityList(r) => r.process(data).await,
            RequestPayload::ActivityDetail(r) => r.process(data).await,
            RequestPayload::ScMyScore(r) => r.process(data).await,
            RequestPayload::ScMyActivity(r) => r.process(data).await,
            RequestPayload::MajorList(r) => r.process(data).await,
            // RequestPayload::ClassList(r) => r.process(data).await,
            // RequestPayload::CourseList(r) => r.process(data).await,
            // RequestPayload::Profile(r) => r.process(data).await,
            RequestPayload::TimeTable(r) => r.process(data).await,
            RequestPayload::Score(r) => r.process(data).await,
            RequestPayload::ScoreDetail(r) => r.process(data).await,
            RequestPayload::SearchLibrary(r) => r.process(data).await,
            RequestPayload::BookHoldingInfo(r) => r.process(data).await,
            RequestPayload::CardExpense(r)=>r.process(data).await,
        }
    }
}
