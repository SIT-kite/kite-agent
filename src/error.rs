pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, thiserror::Error)]
#[error("代理错误: {}", 0)]
pub enum AgentError {
    #[error("无法连接到 kite-server")]
    ConnectionFailure,
    #[error("连接错误: {}", 0)]
    Server(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ZfError {
    #[error("Session error : {0}.")]
    SessionError(String),
    #[error("Can't get public key")]
    PublicKeyError,
}
