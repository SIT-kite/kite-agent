use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::agent::SharedData;
use crate::error::Result;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::parser::Semester;
use crate::service::{DoRequest, ResponsePayload, ResponseResult};
use crate::service::edu::{make_sure_active, url};

#[derive(Debug, Deserialize)]
pub struct ExamArrangeRequest {
    pub account: String,
    pub password: String,
    pub academic_year: u32,
    pub semester: Semester,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamArrangement {
    /// 课程名称
    #[serde(rename(deserialize = "kcmc"))]
    pub course_name: String,
    /// 考试时间
    #[serde(rename(deserialize = "kssj"))]
    pub exam_time: String,
    /// 考试地点
    #[serde(rename(deserialize = "cdmc"))]
    pub exam_location: String,
    /// 考试校区
    #[serde(rename(deserialize = "cdxqmc"))]
    pub exam_campus_name: String,
    /// 课程号
    #[serde(rename(deserialize = "kch"))]
    pub course_id: String,
    /// 重修标记
    #[serde(rename(deserialize = "cxbj"))]
    pub is_retaked: String,
    /// 考试名称
    #[serde(rename(deserialize = "ksmc"))]
    pub exam_name: String,
    /// 考试备注
    #[serde(default, rename(deserialize = "ksbz"))]
    pub exam_tip: String,
    /// 教学班名称
    #[serde(rename(deserialize = "jxbmc"))]
    pub dyn_class_id: String,
    /// 考试方式
    #[serde(default, rename(deserialize = "ksfs"))]
    pub exam_method: String,
    /// 座位号
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
            ("xnm", self.academic_year.to_string()),
            ("xqm", self.semester.to_raw().to_string()),
        ];

        let request = client
            .raw_client
            .post(url::EXAM_ARRANGEMENT)
            .form(&params)
            .build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session)?;

        let text = response.text().await?;
        Ok(ResponsePayload::ExamArrange(parse_exam_arrangement(&text)?))
    }
}

fn parse_exam_arrangement(text: &str) -> Result<Vec<ExamArrangement>> {
    let values: Value = serde_json::from_str(text)?;
    let arrangement_items = values["items"].clone();
    let arrangements: Vec<ExamArrangement> = serde_json::from_value(arrangement_items)?;
    Ok(arrangements)
}
