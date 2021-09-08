use serde::Deserialize;

use crate::agent::SharedData;
use crate::error::Result;
use crate::make_parameter;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::parser::{get_activity_detail, get_score_detail, Activity, ActivityDetail, Parse};
use crate::service::{ActionError, DoRequest, ResponsePayload};

use super::ResponseResult;

const CATEGORY_MAPPING: &[&str] = &[
    "",
    "001",                              // Subject report.(主题报告)
    "8ab17f543fe62d5d013fe62efd3a0002", // Social practice.(社会实践)
    "ff8080814e241104014eb867e1481dc3", // Innovation, entrepreneurship and creativity.(创新创业创意)
    "8F963F2A04013A66E0540021287E4866", // Campus safety and civilization.(校园安全文明)
    "8ab17f543fe62d5d013fe62e6dc70001", // Charity and Volunteer.(公益志愿)
    "8ab17f2a3fe6585e013fe6596c300001", // Campus culture.(校园文化)
    "ff808081674ec4720167ce60dda77cea", // Theme education (主题教育)
    "8ab17f543fe626a8013fe6278a880001", // Yiban Community (易班社区)
    "402881de5d62ba57015d6320f1a7000c", // Safe Online Education (安全网络教育)
    "8ab17f533ff05c27013ff06d10bf0001", // Paper Patent (论文专利)
    "ff8080814e241104014fedbbf7fd329d", // Meeting (会议)
];

mod url {
    pub const SSO_SC_REDIRECT: &str =
        "https://authserver.sit.edu.cn/authserver/login?service=http%3A%2F%2Fsc.sit.edu.cn%2F";

    pub const SCORE_DETAIL: &str = "http://sc.sit.edu.cn/public/pcenter/scoreDetail.action";

    pub const ACTIVITY_DETAIL: &str =
        "http://sc.sit.edu.cn/public/pcenter/activityOrderList.action?pageSize=200";
}

#[derive(Debug, Deserialize)]
pub struct ActivityListRequest {
    /// Count of activities per page.
    pub count: u16,
    /// Page index.
    pub index: u16,
    /// Category Id
    pub category: i32,
}

async fn make_sure_active(client: &mut UserClient) -> Result<()> {
    let home_request = client.raw_client.get(url::SSO_SC_REDIRECT).build()?;
    let response = client.send(home_request).await?;
    if response.url().as_str() == url::SSO_SC_REDIRECT {
        client.login_with_session().await?;
        let request = client.raw_client.get(url::SSO_SC_REDIRECT).build()?;
        let _ = client.send(request).await?;
    }
    Ok(())
}

async fn tran_category(category: i32) -> Result<String> {
    if let Some(category_key) = CATEGORY_MAPPING.get(category as usize) {
        Ok(category_key.to_string())
    } else {
        Err(ActionError::ParsingError.into())
    }
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
        let category_id = tran_category(self.category).await?;
        let request = client
            .raw_client
            .get(&format!(
                "http://sc.sit.edu.cn/public/activity/activityList.action?{}",
                make_parameter!("pageNo" => &self.index.to_string(),"pageSize" => &self.count.to_string(),
                    "categoryId" => category_id.as_str()
                )
            ))
            .build()?;
        let response = client.send(request).await?;

        data.session_store.insert(&client.session)?;

        let html = response.text().await?;
        let mut activities: Vec<Activity> = Parse::from_html(&html)?;
        let result: Vec<Activity> = activities
            .into_iter()
            .map(|mut s| {
                s.category = self.category;
                s
            })
            .collect();
        Ok(ResponsePayload::ActivityList(result))
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

#[derive(Debug, Deserialize)]
pub struct ScScoreItemRequest {
    pub account: String,
    pub password: String,
}

#[async_trait::async_trait]
impl DoRequest for ScScoreItemRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let request = client.raw_client.get(url::SCORE_DETAIL).build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        data.session_store.insert(&client.session)?;

        let score = get_score_detail(&html)?;
        Ok(ResponsePayload::ScScoreDetail(score))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScActivityRequest {
    pub account: String,
    pub password: String,
}

#[async_trait::async_trait]
impl DoRequest for ScActivityRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        let request = client.raw_client.get(url::ACTIVITY_DETAIL).build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        data.session_store.insert(&client.session)?;

        let activity = get_activity_detail(&html)?;
        Ok(ResponsePayload::ScActivityDetail(activity))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScJoinRequest {
    pub account: String,
    pub password: String,
    pub activity_id: i32,
    pub force: bool,
}

#[async_trait::async_trait]
impl DoRequest for ScJoinRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);
        client.set_response_hook(Some(default_response_hook));

        make_sure_active(&mut client).await?;

        // Expected page content
        let _expected = "<script>alert('申请成功，下面将为您跳转至我的活动页面！');location.href='/public/pcenter/activityOrderList.action'</script>";
        let request = client.raw_client.get(url::ACTIVITY_DETAIL).build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        data.session_store.insert(&client.session)?;

        let activity = get_activity_detail(&html)?;
        Ok(ResponsePayload::ScActivityDetail(activity))
    }
}
