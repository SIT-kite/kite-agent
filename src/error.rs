use awc::http::StatusCode;
use awc::error::SendRequestError;

pub type Result<T> = std::result::Result<T, CrawlerError>;

pub enum CrawlerError {
    HttpError(StatusCode),
    ConnectionError(String),
}


impl From<SendRequestError> for CrawlerError {
    fn from(request_err: SendRequestError) -> Self {
        CrawlerError::ConnectionError(request_err.to_string())
    }
}