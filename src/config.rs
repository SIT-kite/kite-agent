use std::{error::Error, fs};

use serde::Deserialize;

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

pub(crate) const USERAGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 ' \
'Safari/537.36 Edg/87.0.664.66";
