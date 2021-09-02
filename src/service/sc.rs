use super::edu::url;
use super::ResponseResult;
use crate::agent::SharedData;
use crate::error::Result;
use crate::make_parameter;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::parser::{Activity, ActivityDetail, Parse};
use crate::service::{ActionError, DoRequest, ResponsePayload};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivityListRequest {
    /// Count of activities per page.
    pub count: u16,
    /// Page index.
    pub index: u16,
}

const COOKIE_PAGE: &str =
    "https://authserver.sit.edu.cn/authserver/login?service=http%3A%2F%2Fsc.sit.edu.cn%2F";

const SCORE_DETAIL: &str = "http://sc.sit.edu.cn/public/pcenter/scoreDetail.action";

async fn make_sure_active(client: &mut UserClient) -> Result<()> {
    let home_request = client.raw_client.get(COOKIE_PAGE).build()?;
    let response = client.send(home_request).await?;
    if response.url().as_str() == COOKIE_PAGE {
        client.login_with_session().await?;
        let request = client.raw_client.get(url::SSO_REDIRECT).build()?;
        let _ = client.send(request).await?;
    }
    Ok(())
}

#[async_trait::async_trait]
impl DoRequest for ActivityListRequest {
    /// Fetch and parse activity list page.
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data
            .session_store
            .choose_randomly()?
            .ok_or(ActionError::NoSessionAvailable)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let request = client
            .raw_client
            .get(&format!(
                "http://sc.sit.edu.cn/public/activity/activityList.action?{}",
                make_parameter!("pageNo" => &self.index.to_string(),"pageSize" => &self.count.to_string(),
                    "categoryId" => "",
                    "activityName" => ""
                )
            ))
            .build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session)?;

        let html = response.text().await?;
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
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data
            .session_store
            .choose_randomly()?
            .ok_or(ActionError::NoSessionAvailable)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let request = client
            .raw_client
            .get(&format!(
                "http://sc.sit.edu.cn/public/activity/activityDetail.action?activityId={}",
                self.id
            ))
            .build()?;
        let response = client.send(request).await?;

        let html = response.text().await?;

        data.session_store.insert(&client.session)?;

        let activity: ActivityDetail = Parse::from_html(&html)?;
        Ok(ResponsePayload::ActivityDetail(Box::from(activity)))
    }
}
