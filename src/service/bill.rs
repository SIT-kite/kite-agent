use super::ResponseResult;
use crate::communication::AgentData;
use crate::make_parameter;
use crate::parser::{ElectricityBill, Parse};
use crate::service::{ActionError, ResponsePayload};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ElectricityBillRequest {
    pub room: String,
}

impl ElectricityBillRequest {
    pub async fn process(self, parameter: AgentData) -> ResponseResult {
        let mut session_storage = parameter.parameter;
        let session = session_storage
            .choose_randomly()?
            .ok_or(ActionError::NoSessionAvailable)?;

        let cookie = session.get_cookie_string("card.sit.edu.cn");
        let http_response = reqwest::Client::new()
            .post("http://card.sit.edu.cn/dk_xxmh.jsp")
            .header("Cookie", cookie)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(make_parameter!(
                "actionType" => "init",
                "selectstate" => "on",
                "fjh" => &self.room
            ))
            .send()
            .await?;

        let html_page = http_response.text_with_charset("GB2312").await.unwrap();
        Ok(ResponsePayload::ElectricityBill(ElectricityBill::from_html(
            &html_page,
        )))
    }
}
