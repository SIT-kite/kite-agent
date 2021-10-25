use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::agent::SharedData;
use crate::error::Result;
use crate::net::client::default_response_hook;
use crate::net::UserClient;
use crate::parser::{ExpensePage, Parse};
use crate::service::{DoRequest, ResponsePayload, ResponseResult};

mod url {
    use const_format::concatcp;

    pub const OA_HOME: &str = "https://myportal.sit.edu.cn/";
    pub const CARD_HOME: &str = "http://card.sit.edu.cn";
    pub const EXPENSE_PAGE: &str = concatcp!(CARD_HOME, "/personalxiaofei.jsp");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseRequest {
    /// 账户
    pub account: String,
    /// 密码
    pub password: String,
    /// 页号
    pub page: Option<u32>,
    /// 起始时间
    pub start_time: Option<String>,
    /// 终止时间
    pub end_time: Option<String>,
}

impl ExpenseRequest {
    pub fn build_url(&self) -> Url {
        let mut params: Vec<(&str, String)> = vec![];

        // self.page.and_then(|x|  params.push(("page", x.to_string())));
        if let Some(p) = self.page {
            params.push(("page", p.to_string()));
        }
        if let Some(st) = self.start_time.clone() {
            params.push(("from", st));
        }
        if let Some(et) = self.end_time.clone() {
            params.push(("to", et));
        }

        Url::parse_with_params(url::EXPENSE_PAGE, params.iter()).unwrap()
    }
}

async fn make_sure_active(client: &mut UserClient) -> Result<()> {
    // If OA home is accessible, card home is ensured to be accessed.
    let home_request = client.raw_client.get(url::OA_HOME).build()?;
    let response = client.send(home_request).await?;

    if response.url().path() != "/" {
        // The session is already expired, re-login now.
        client.login_with_session().await?;

        let home_request = client.raw_client.get(url::OA_HOME).build()?;
        let _ = client.send(home_request).await?;
    }
    Ok(())
}

#[async_trait::async_trait]
impl DoRequest for ExpenseRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = data.session_store.query_or(&self.account, &self.password)?;
        let mut client = UserClient::new(session, &data.client);

        client.set_response_hook(Some(default_response_hook));
        make_sure_active(&mut client).await?;

        data.session_store.insert(&client.session)?;

        let request = client.raw_client.get(self.build_url()).build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        let expense_page = ExpensePage::from_html(&html)?;
        Ok(ResponsePayload::CardExpense(expense_page))
    }
}
