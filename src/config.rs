use serde::Deserialize;
use std::{error::Error, fs};

const DEFAULT_CONFIG_PATH: &str = "kite.toml";

lazy_static! {
    /// Global configuration
    pub static ref CONFIG: Config = load_config(DEFAULT_CONFIG_PATH)
        .map_err(|e| panic!("Failed to load {}: {}", DEFAULT_CONFIG_PATH, e))
        .unwrap();
}

#[derive(Deserialize)]
/// Global agent configs.
pub struct Config {
    /// Agent basic configuration.
    pub agent: AgentConfig,
    /// Server related.
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    /// Server address string. e.g, "wss://localhost/ag"
    pub addr: String,
    ///  Max connections to server.
    pub conn: u8,
}

#[derive(Deserialize)]
pub struct AgentConfig {
    /// Agent identified name
    pub name: String,
    /// DB name
    pub db: String,
    /// Proxy string for most connections.
    pub proxy: Option<String>,
}

/// Load the global configuration from DEFAULT_CONFIG_PATH on the startup.
fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let toml = toml::from_str(&text)?;

    Ok(toml)
}

/// URL that probably used in the program
pub(crate) mod url {
    use const_format::concatcp;

    /// Server address for 正方教务系统
    pub const HOME: &str = "http://jwxt.sit.edu.cn";

    /* Login related */

    pub const LOGIN: &str = concatcp!(HOME, "/jwglxt/xtgl/login_slogin.html");
    pub const RSA_PUBLIC_KEY: &str = concatcp!(HOME, "/jwglxt/xtgl/login_getPublicKey.html");

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

pub(crate) const USERAGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 ' \
'Safari/537.36 Edg/87.0.664.66";
