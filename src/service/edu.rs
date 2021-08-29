pub use env::{ClassRequest, CourseRequest, MajorRequest};
pub use user::{ProfileRequest, ScoreRequest, TimeTableRequest};

use crate::error::Result;
use crate::net::UserClient;

mod auth;
mod env;
mod user;

/// URL probably used in the module.
pub(crate) mod url {
    use const_format::concatcp;

    /// Server address for 正方教务系统
    pub const HOME: &str = "http://jwxt.sit.edu.cn";

    /* Login related */

    pub const LOGIN: &str = concatcp!(HOME, "/jwglxt/xtgl/login_slogin.html");
    pub const RSA_PUBLIC_KEY: &str = concatcp!(HOME, "/jwglxt/xtgl/login_getPublicKey.html");
    pub const SSO_REDIRECT: &str = "https://authserver.sit.edu.cn/authserver/login?service=http%3A%2F%2Fjwxt.sit.edu.cn%2Fsso%2Fjziotlogin";

    /* function related */

    /// Score list page
    pub const SCORE_LIST: &str = concatcp!(
        HOME,
        "/jwglxt/cjcx/cjcx_cxDgXscj.html?doType=query&gnmkdm=N305005"
    );
    /// Time tanle page
    pub const TIME_TABLE: &str = concatcp!(HOME, "/jwglxt/kbcx/xskbcx_cxXsKb.html?gnmkdm=N253508");
    /// Personal profile page
    pub const PROFILE: &str = concatcp!(
        HOME,
        "/jwglxt/xsxxxggl/xsgrxxwh_cxXsgrxx.html?gnmkdm=N100801&layout=default"
    );
    /// Major list page
    pub const MAJOR_LIST: &str = concatcp!(HOME, "/jwglxt/xtgl/comm_cxZyfxList.html?gnmkdm=N214505");
    /// Class list page
    pub const CLASS_LIST: &str = concatcp!(HOME, "/jwglxt/xtgl/comm_cxBjdmList.html?gnmkdm=N214505");
    /// Suggested course and time table
    pub const SUGGESTED_COURSE: &str = concatcp!(HOME, "/jwglxt/kbdy/bjkbdy_cxBjKb.html?gnmkdm=N214505");
}

async fn make_sure_active(client: &mut UserClient) -> Result<()> {
    let home_request = client.raw_client.get(url::HOME).build()?;
    let response = client.send(home_request).await?;

    if response.url().as_str() == url::LOGIN {
        // The session is already expired, re-login now.
        client.login_with_session().await?;

        // Use SSO to Zhengfang system.
        let request = client.raw_client.get(url::SSO_REDIRECT).build()?;
        let _ = client.send(request).await?;
    }
    Ok(())
}
