use std::collections::HashMap;

use async_trait::async_trait;
use serde::Deserialize;

use crate::agent::SharedData;
use crate::config::url::{CLASS_LIST, MAJOR_LIST, PROFILE, SCORE_LIST, SUGGESTED_COURSE, TIME_TABLE};
use crate::config::USERAGENT;
use crate::error::{Result, ZfError};
use crate::net::client::send_request;
use crate::net::{ClientBuilder, Session, UserClient};
use crate::parser::*;
use crate::service::edu::auth::login;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};

async fn get_timetable(
    mut client: UserClient,
    school_year: SchoolYear,
    semester: Semester,
) -> Result<Vec<Course>> {
    let data = [
        ("xnm", school_year.to_string()),
        ("xqm", semester.to_raw().to_string()),
    ];

    let reqest = reqwest::Request::new(reqwest::Method::POST, TIME_TABLE.parse()?);
    let response = client.send_request(request).await?;
    let page = self.post_url(TIME_TABLE, &data).await?;
    let text = page.text().await?;
    parse_timetable_page(&text)
}

async fn get_score_list(
    mut client: UserClient,
    school_year: SchoolYear,
    semester: Semester,
) -> Result<Vec<Score>> {
    let data = [
        ("xnm", school_year.to_string()),
        ("xqm", semester.to_raw().to_string()),
        ("queryModel.showCount", "5000".to_string()),
    ];
    let page = self.post_url(SCORE_LIST, &data).await?;
    let text = page.text().await?;
    parse_score_list_page(&text)
}

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub account: String,
    pub passwd: String,
}

#[async_trait]
impl DoRequest for ProfileRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = match data.session.query(&self.account)? {
            Some(session) => session,
            None => {
                let new_session = Session::new(&self.account, &self.passwd);
                // 登录逻辑
                login(&new_session, data.client); //TODO: 将login改成这两个参数
                                                  // 记得要保存session
                data.session.insert(&new_session);
                new_session
            }
        };

        let request = data.client.get(PROFILE).build().unwrap(); // TODO: 去掉 unwrap
        let response = send_request(session, client, request).await?;

        let text = response.text().await?;
        let profile = parse_profile_page(&text)?;

        Ok(ResponsePayload::Profile(profile))
    }
}

#[derive(Debug, Deserialize)]
pub struct TimeTableRequest {
    pub account: String,
    pub passwd: String,
    pub school_year: SchoolYear,
    pub semester: Semester,
}

#[async_trait]
impl DoRequest for TimeTableRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let timetable = client.get_timetable(self.school_year, self.semester).await?;
                Ok(ResponsePayload::TimeTable(timetable))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let timetable = client.get_timetable(self.school_year, self.semester).await?;
                Ok(ResponsePayload::TimeTable(timetable))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScoreRequest {
    pub account: String,
    pub passwd: String,
    pub school_year: SchoolYear,
    pub semester: Semester,
}

#[async_trait]
impl DoRequest for ScoreRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let score = client.get_score_list(self.school_year, self.semester).await?;
                Ok(ResponsePayload::Score(score))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let score = client.get_score_list(self.school_year, self.semester).await?;
                Ok(ResponsePayload::Score(score))
            }
        }
    }
}
