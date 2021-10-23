use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::agent::SharedData;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};
use crate::service::edu::{make_sure_active, url};
use crate::error::Result;
use serde_json::value::Value;

#[derive(Debug, Deserialize)]
pub struct ExamArrangeRequest {
    pub account: String,
    pub password: String,
    pub academic_year: u32,
    pub semester: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamArrangement {
    #[serde(rename(deserialize = "kcmc"))]
    pub course_name: String,
    #[serde(rename(deserialize = "kssj"))]
    pub exam_time: String,
    #[serde(rename(deserialize = "cdmc"))]
    pub exam_location: String,
    #[serde(rename(deserialize = "cdxqmc"))]
    pub exam_campus_name: String,
    #[serde(rename(deserialize = "kch"))]
    pub course_id: String,
    #[serde(rename(deserialize = "cxbj"))]
    pub is_retaked: String,
    #[serde(rename(deserialize = "ksmc"))]
    pub exam_name: String,
    #[serde(rename(deserialize = "ksbz"))]
    pub exam_tip: Option<String>,
    #[serde(rename(deserialize = "jxbmc"))]
    pub class_name: String,
    #[serde(rename(deserialize = "ksfs"))]
    pub exam_method: String,
    #[serde(rename(deserialize = "zwh"))]
    pub exam_seat: String,
}

#[async_trait]
impl DoRequest for ExamArrangeRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let params = [
            ("xnm", self.academic_year),
            ("xqm", self.semester),
        ];

        let request = client.raw_client.post(url::EXAM_ARRANGEMENT).form(&params).build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session)?;

        let text = response.text().await?;
        Ok(ResponsePayload::ExamArrange(parse_exam_arrangement(&text)?))
    }
}

fn parse_exam_arrangement(text: &str) -> Result<Vec<ExamArrangement>> {
    let values: Value = serde_json::from_str(text)?;
    let arrangement_items = values["items"].clone();
    let arrangements : Vec<ExamArrangement> = serde_json::from_value(arrangement_items)?;
    Ok(arrangements)
}
