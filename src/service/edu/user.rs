use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::agent::SharedData;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::parser::*;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};

use super::make_sure_active;
use super::url;

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub account: String,
    pub password: String,
}

// #[async_trait]
// impl DoRequest for ProfileRequest {
//     async fn process(self, mut data: SharedData) -> ResponseResult {
//         let session = data
//             .session_store
//             .query(&self.account)?
//             .unwrap_or_else(|| Session::new(&self.account, &self.password));
//         let mut client = UserClient::new(session, &data.client);
//         client.set_response_hook(Some(default_response_hook));
//
//         make_sure_active(&mut client).await?;
//
//         let request = data.client.get(url::PROFILE).build()?;
//         let response = client.send(request).await?;
//
//         // Save session after the last response is received.
//         data.session_store.insert(&client.session);
//
//         let text = response.text().await?;
//         let profile = parse_profile_page(&text)?;
//         Ok(ResponsePayload::Profile(profile))
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeTableRequest {
    pub account: String,
    pub password: String,
    pub school_year: SchoolYear,
    pub semester: Semester,
}

#[async_trait]
impl DoRequest for TimeTableRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let params = [
            ("xnm", self.school_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
        ];

        let request = client.raw_client.post(url::TIME_TABLE).form(&params).build()?;
        let response = client.send(request).await?;

        // Save session after the last response is received.
        data.session_store.insert(&client.session)?;

        let text = response.text().await?;
        Ok(ResponsePayload::TimeTable(parse_timetable_page(&text)?))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScoreRequest {
    pub account: String,
    pub password: String,
    pub school_year: SchoolYear,
    pub semester: Semester,
}

#[async_trait]
impl DoRequest for ScoreRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let params = [
            ("xnm", self.school_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
            ("queryModel.showCount", "5000".to_string()),
        ];

        let request = data.client.post(url::SCORE_LIST).form(&params).build()?;
        let response = client.send(request).await?;

        // Save session after the last response is received.
        data.session_store.insert(&client.session)?;

        let text = response.text().await?;
        Ok(ResponsePayload::Score(parse_score_list_page(&text)?))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScoreDetailRequest {
    pub account: String,
    pub password: String,
    pub school_year: SchoolYear,
    pub semester: Semester,
    pub class_id: String,
}

#[async_trait]
impl DoRequest for ScoreDetailRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let params = [
            ("jxb_id", self.class_id),
            ("xnm", self.school_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
        ];

        let request = data.client.post(url::SCORE_DETAIL).form(&params).build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        data.session_store.insert(&client.session)?;

        let score_detail = get_score_detail(&html)?;
        Ok(ResponsePayload::ScoreDetail(score_detail))
    }
}
