use crate::communication::{AgentData, Response, ResponsePayload};
use crate::make_parameter;
use crate::net::{domain, Client, ClientBuilder};
use crate::parser::{Activity, Parse};
use reqwest::{Response as HttpResponse, StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActivityListRequest {
    // pub account: String,
    // pub credential: String,
    /// Count of activities per page.
    pub count: u16,
    /// Page index.
    pub index: u16,
}

const COOKIE_PAGE: &str =
    "https://authserver.sit.edu.cn/authserver/login?service=http%3A%2F%2Fsc.sit.edu.cn%2F";

impl ActivityListRequest {
    async fn get_with_auto_redirect(client: &mut Client, start_page: &str) -> HttpResponse {
        let mut remain_redirect = 10;
        let mut next_hop = start_page.to_string();
        let mut response = client.get(&next_hop).send().await.unwrap();

        while remain_redirect > 0 && response.status() == StatusCode::FOUND {
            let redirect_url = response.headers().get("location");
            if redirect_url.is_none() {
                return response;
            }
            let t = redirect_url.unwrap().to_str().unwrap().to_string();
            next_hop = if domain(&t).is_none() {
                format!("http://{}/{}", domain(&next_hop).unwrap(), t)
            } else {
                t
            };
            response = client.get(&next_hop).send().await.unwrap();
            remain_redirect -= 1;
        }
        response
    }

    pub async fn process(self, parameter: AgentData) -> Response {
        let mut session_storage = parameter.parameter;
        let session = session_storage.choose_randomly().unwrap();

        if session.is_none() {
            return Response::error(10);
        }
        let session = session.unwrap();
        let mut client = ClientBuilder::new(session)
            .redirect(false)
            .proxy("http://127.0.0.1:8888/")
            .build();

        let mut try_count = 2;
        let mut html = String::new();

        while try_count > 0 {
            let t = client.session().query_cookie("sc.sit.edu.cn", "JSESSIONID");
            if t.is_none() {
                Self::get_with_auto_redirect(&mut client, COOKIE_PAGE).await;
            }

            let response = client
                .get(&format!(
                    "http://sc.sit.edu.cn/public/activity/activityList.action?{}",
                    make_parameter!(
                    "pageNo" => &self.index.to_string(),
                    "pageSize" => &self.count.to_string(),
                    "categoryId" => "",
                    "activityName" => ""
                    )
                ))
                .send()
                .await
                .unwrap();

            html = response.text().await.unwrap();
            if html.starts_with("<script languge='javascript'>") && html.len() < 500 {
                client.session_mut().login().await;
            }
            try_count -= 1;
        }
        session_storage.insert(client.session());

        let activities: Vec<Activity> = Parse::from_html(&html);
        Response::normal(ResponsePayload::ActivityList(activities))
    }
}
