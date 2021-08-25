mod environment;
mod user;

use crate::zftool::config::USERAGENT;
use crate::zftool::session::ZfSession;
use crate::zftool::Result;
pub use environment::Environment;
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::Response;
pub use user::User;

#[derive(Debug)]
pub struct ZfClient {
    pub(crate) user: String,
    pub(crate) session: ZfSession,
}

impl ZfClient {
    async fn get_url(&mut self, url: &str, data: &[(&str, String)]) -> Result<Response> {
        let response = self
            .session
            .client
            .get(url)
            .form(data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        self.session.sync_cookies("jwxt.sit.edu.cn", response.cookies());
        Ok(response)
    }

    async fn post_url(&mut self, url: &str, data: &[(&str, String)]) -> Result<Response> {
        let response = self
            .session
            .client
            .post(url)
            .form(data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        self.session.sync_cookies("jwxt.sit.edu.cn", response.cookies());
        Ok(response)
    }
}
