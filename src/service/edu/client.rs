use crate::agent::SharedData;
use crate::config::url::{CLASS_LIST, MAJOR_LIST, PROFILE, SCORE_LIST, SUGGESTED_COURSE, TIME_TABLE};
use crate::config::USERAGENT;
use crate::error::Result;
use crate::net::client::Client;
use crate::net::{ClientBuilder, Session};
use crate::parser::*;
use crate::service::edu::login;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};
use async_trait::async_trait;
use reqwest::header::{COOKIE, USER_AGENT};
use serde::Deserialize;
use std::collections::HashMap;

impl Client {
    async fn get_major_list(&mut self, entrance_year: SchoolYear) -> Result<Vec<Major>> {
        let param = [("njdm_id", entrance_year.to_string())];
        let page = self.get_url(MAJOR_LIST, &param).await?;
        let text = page.text().await?;
        parse_major_list_page(&text)
    }

    async fn get_class_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Class>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("queryModel.showCount", 10000.to_string()),
        ];
        let page = self.post_url(CLASS_LIST, &data).await?;
        let text = page.text().await?;
        parse_class_list_page(&text)
    }

    async fn get_suggested_course_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
        major_id: &str,
        class_id: &str,
        entrance_year: Option<&str>,
    ) -> Result<Vec<Course>> {
        let mut year;
        match entrance_year {
            Some(x) => {
                year = x.to_string();
            }
            None => {
                year = "20".to_string();
                let classes = class_id.chars();
                let mut count = 0;
                for text in classes {
                    year += &text.to_string();
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("njdm_id", year),
            ("zyh_id", major_id.to_string()),
            ("bh_id", class_id.to_string()),
            ("tjkbzdm", "1".to_string()),
            ("tjkbzxsdm", "0".to_string()),
        ];
        let page = self.post_url(SUGGESTED_COURSE, &data).await?;
        let text = page.text().await?;
        parse_timetable_page(&text)
    }

    async fn get_profile(&mut self) -> Result<Profile> {
        let page = self
            .client
            .get(PROFILE)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        parse_profile_page(&text)
    }

    async fn get_timetable(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Course>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
        ];
        let page = self.post_url(TIME_TABLE, &data).await?;
        let text = page.text().await?;
        parse_timetable_page(&text)
    }

    async fn get_score_list(
        &mut self,
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
}

#[derive(Debug, Deserialize)]
pub struct MajorRequest {
    pub entrance_year: SchoolYear,
    pub account: String,
    pub passwd: String,
}

#[async_trait]
impl DoRequest for MajorRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let major = client.get_major_list(self.entrance_year).await?;
                Ok(ResponsePayload::MajorList(major))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let major = client.get_major_list(self.entrance_year).await?;
                Ok(ResponsePayload::MajorList(major))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClassRequest {
    pub entrance_year: SchoolYear,
    pub semester: Semester,
    pub account: String,
    pub passwd: String,
}

#[async_trait]
impl DoRequest for ClassRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let class = client.get_class_list(self.entrance_year, self.semester).await?;
                Ok(ResponsePayload::ClassList(class))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let class = client.get_class_list(self.entrance_year, self.semester).await?;
                Ok(ResponsePayload::ClassList(class))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CourseRequest {
    pub school_year: SchoolYear,
    pub semester: Semester,
    pub account: String,
    pub passwd: String,
    pub major_id: String,
    pub class_id: String,
    pub entrance_year: Option<String>,
}

#[async_trait]
impl DoRequest for CourseRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        let entrance_year = self.entrance_year.as_deref();
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let course = client
                    .get_suggested_course_list(
                        self.school_year,
                        self.semester,
                        self.major_id.as_str(),
                        self.class_id.as_str(),
                        entrance_year,
                    )
                    .await?;
                Ok(ResponsePayload::CourseList(course))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let course = client
                    .get_suggested_course_list(
                        self.school_year,
                        self.semester,
                        self.major_id.as_str(),
                        self.class_id.as_str(),
                        entrance_year,
                    )
                    .await?;
                Ok(ResponsePayload::CourseList(course))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub account: String,
    pub passwd: String,
}

#[async_trait]
impl DoRequest for ProfileRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let session = data.session.query(self.account.as_str())?;
        match session {
            // If the session is available, should first verify the password is true or not
            Some(s) => {
                let mut client = ClientBuilder::new(s).build();
                let profile = client.get_profile().await?;
                Ok(ResponsePayload::Profile(profile))
            }
            None => {
                let s = Session::new(self.account.as_str(), self.passwd.as_str());
                let mut client = ClientBuilder::new(s).build();
                login(client);
                let profile = client.get_profile().await?;
                Ok(ResponsePayload::Profile(profile))
            }
        }
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
