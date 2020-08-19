use crate::service::ActionError;
use bincode::Error as BincodeError;
use reqwest::Error as ReqwestError;
use sled::Error as SledError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AgentError>;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("网络连接异常: {0}")]
    Connection(String),
    #[error("解析Html出错, {0}")]
    HtmlParser(String),
    #[error("数据库错误: {0}")]
    DbError(String),
    #[error("内部解析错误: {0}")]
    BincodeError(String),
    #[error("{0}")]
    ActionError(ActionError),
}

impl From<SledError> for AgentError {
    fn from(err: SledError) -> Self {
        AgentError::DbError(err.to_string())
    }
}

impl From<BincodeError> for AgentError {
    fn from(err: BincodeError) -> Self {
        AgentError::BincodeError(err.to_string())
    }
}

impl From<ActionError> for AgentError {
    fn from(err: ActionError) -> Self {
        AgentError::ActionError(err)
    }
}

impl From<ReqwestError> for AgentError {
    fn from(err: ReqwestError) -> Self {
        AgentError::Connection(err.to_string())
    }
}
