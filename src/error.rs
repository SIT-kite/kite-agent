use awc::error::SendRequestError;
use awc::http::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CrawlerError>;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Http响应异常: {0}")]
    Http(StatusCode),
    #[error("网络连接异常: {0}")]
    Connection(String),
    #[error("解析Html出错, {0}")]
    HtmlParser(String),
}

impl From<SendRequestError> for CrawlerError {
    fn from(request_err: SendRequestError) -> Self {
        CrawlerError::Connection(request_err.to_string())
    }
}
