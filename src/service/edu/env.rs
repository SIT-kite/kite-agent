use async_trait::async_trait;
use serde::Deserialize;

use super::url;
use crate::agent::SharedData;
use crate::net::client::default_response_hook;
use crate::net::{Session, UserClient};
use crate::parser::*;
use crate::service::edu::make_sure_active;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};

#[derive(Debug, Deserialize)]
pub struct ClassRequest {
    pub school_year: SchoolYear,
    pub semester: Semester,
    pub account: String,
    pub passwd: String,
}

#[async_trait]
impl DoRequest for ClassRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data
            .session_store
            .query(&self.account)?
            .unwrap_or_else(|| Session::new(&self.account, &self.passwd));
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let params = [
            ("xnm", self.school_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
            ("queryModel.showCount", 10000.to_string()),
        ];

        let request = client.raw_client.post(url::CLASS_LIST).form(&params).build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session);

        let text = response.text().await?;
        Ok(ResponsePayload::ClassList(parse_class_list_page(&text)?))
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
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data
            .session_store
            .query(&self.account)?
            .unwrap_or_else(|| Session::new(&self.account, &self.passwd));
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let mut year;
        match self.entrance_year {
            Some(x) => {
                year = x.to_string();
            }
            None => {
                year = "20".to_string();
                let classes = self.class_id.chars();
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
        let params = [
            ("xnm", self.school_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
            ("njdm_id", year),
            ("zyh_id", self.major_id.to_string()),
            ("bh_id", self.class_id.to_string()),
            ("tjkbzdm", "1".to_string()),
            ("tjkbzxsdm", "0".to_string()),
        ];

        let request = client
            .raw_client
            .post(url::SUGGESTED_COURSE)
            .form(&params)
            .build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session);

        let text = response.text().await?;
        Ok(ResponsePayload::CourseList(parse_timetable_page(&text)?))
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
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data
            .session_store
            .query(&self.account)?
            .unwrap_or_else(|| Session::new(&self.account, &self.passwd));
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let request = client.raw_client.get(url::MAJOR_LIST).build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session);

        let text = response.text().await?;
        Ok(ResponsePayload::MajorList(parse_major_list_page(&text)?))
    }
}
