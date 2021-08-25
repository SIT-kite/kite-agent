pub use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum ZfError {
    #[error("Session error : {0}.")]
    SessionError(String),
    #[error("Can't get public key")]
    PublicKeyError,
}
