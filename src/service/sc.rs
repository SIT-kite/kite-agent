use reqwest::{Response as HttpResponse, StatusCode};
use serde::Deserialize;

use crate::agent::SharedData;
use crate::make_parameter;
use crate::net::{domain, ClientBuilder, UserClient};
use crate::parser::{Activity, ActivityDetail, Parse};
use crate::service::{ActionError, DoRequest, ResponsePayload};

use super::ResponseResult;

#[derive(Debug, Deserialize)]
pub struct ActivityListRequest {
    /// Count of activities per page.
    pub count: u16,
    /// Page index.
    pub index: u16,
}

const COOKIE_PAGE: &str =
    "https://authserver.sit.edu.cn/authserver/login?service=http%3A%2F%2Fsc.sit.edu.cn%2F";

async fn get_with_auto_redirect(client: &mut UserClient, start_page: &str) -> HttpResponse {
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

#[async_trait::async_trait]
impl DoRequest for ActivityListRequest {
    /// Fetch and parse activity list page.
    async fn process(self, data: SharedData) -> ResponseResult {
        let mut session_storage = data.session;
        let session = session_storage
            .choose_randomly()?
            .ok_or(ActionError::NoSessionAvailable)?;
        let mut client = ClientBuilder::new(session).redirect(false).build();

        let mut try_count = 2;
        let mut html = String::new();

        while try_count > 0 {
            let t = client.session().query_cookie("sc.sit.edu.cn", "JSESSIONID");
            if t.is_none() {
                get_with_auto_redirect(&mut client, COOKIE_PAGE).await;
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
                .await?;

            html = response.text().await?;
            // Note: the server do return "languge" rather than "language"
            if html.starts_with("<script languge='javascript'>") && html.len() < 500 {
                client.session_mut().login().await?;
            } else {
                break;
            }
            try_count -= 1;
        }
        session_storage.insert(client.session())?;

        let activities: Vec<Activity> = Parse::from_html(&html)?;
        Ok(ResponsePayload::ActivityList(activities))
    }
}

#[derive(Debug, Deserialize)]
pub struct ActivityDetailRequest {
    /// Activity id in sc.sit.edu.cn
    pub id: String,
}

#[async_trait::async_trait]
impl DoRequest for ActivityDetailRequest {
    /// Fetch and parse activity detail page.
    async fn process(self, data: SharedData) -> ResponseResult {
        let mut session_storage = data.session;
        let session = session_storage
            .choose_randomly()?
            .ok_or(ActionError::NoSessionAvailable)?;
        let mut client = ClientBuilder::new(session).redirect(false).build();

        let mut try_count = 2;
        let mut html = String::new();

        while try_count > 0 {
            let t = client.session().query_cookie("sc.sit.edu.cn", "JSESSIONID");
            if t.is_none() {
                get_with_auto_redirect(&mut client, COOKIE_PAGE).await;
            }

            let response = client
                .get(&format!(
                    "http://sc.sit.edu.cn/public/activity/activityDetail.action?activityId={}",
                    self.id
                ))
                .send()
                .await?;

            html = response.text().await?;
            if html.starts_with("<script languge='javascript'>") && html.len() < 500 {
                client.session_mut().login().await?;
            } else {
                break;
            }
            try_count -= 1;
        }
        session_storage.insert(client.session())?;

        let activity: ActivityDetail = Parse::from_html(&html)?;
        Ok(ResponsePayload::ActivityDetail(Box::from(activity)))
    }
}
