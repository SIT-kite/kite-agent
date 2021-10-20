use serde::{Deserialize, Serialize};

use reqwest::Url;
use crate::agent::SharedData;
use crate::net::{UserClient};
use crate::parser::{ExpensePage, Parse};
use crate::service::{DoRequest, ResponsePayload, ResponseResult};


mod url {
    use const_format::concatcp;

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

        Url::parse_with_params(
            url::EXPENSE_PAGE,
            params.iter(),
        ).unwrap()
    }
}

#[async_trait::async_trait]
impl DoRequest for ExpenseRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        // 查询本地的登录缓存，没有就构造登录缓存
        let session = data.session_store.query_or(&self.account, &self.password)?;

        // 创建client
        let mut client = UserClient::new(session, &data.client);

        client.login_with_session().await?;

        // client.set_response_hook(Some(default_response_hook));
        let request = client
            .raw_client
            .get(self.build_url())
            .build()?;
        let response = client.send(request).await?;
        let html = response.text().await?;

        let expense_page = ExpensePage::from_html(&html).unwrap();
        Ok(ResponsePayload::CardExpense(expense_page))
    }
}