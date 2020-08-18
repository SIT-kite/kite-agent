use crate::communication::{AgentData, Response, ResponsePayload};
use crate::make_parameter;
use crate::parser::{ElectricityBill, Parse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ElectricityBillRequest {
    pub room: String,
}

impl ElectricityBillRequest {
    pub async fn process(self, parameter: AgentData) -> Response {
        let mut session_storage = parameter.parameter;
        let session = session_storage.choose_randomly().unwrap();

        if let Some(session) = session {
            let cookie = session.get_cookie_string("card.sit.edu.cn");
            let http_response = reqwest::Client::new()
                .post("http://card.sit.edu.cn/dk_xxmh.jsp")
                .header("Cookie", cookie)
                .body(make_parameter!(
                    "actionType" => "init",
                    "selectstate" => "on",
                    "fjh" => &self.room
                ))
                .send()
                .await;

            match http_response {
                Ok(content) => {
                    let html_page = content.text_with_charset("GB2312").await.unwrap();
                    return Response::normal(ResponsePayload::ElectricityBill(
                        ElectricityBill::from_html(&html_page),
                    ));
                }
                Err(_) => (),
            };
        }
        Response::error(11)
    }
}
