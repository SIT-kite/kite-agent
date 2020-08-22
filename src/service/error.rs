use super::{Response, ResponsePayload};
use num_traits::ToPrimitive;

#[derive(Debug, thiserror::Error, ToPrimitive)]
/// ActionError, is used to transfer error in common, or not critical.
pub enum ActionError {
    #[error("Invalid request payload.")]
    BadRequest = 2,
    #[error("用户名或密码错误")]
    LoginFailed = 50,
    #[error("找不到可用的会话")]
    NoSessionAvailable = 51,
}

/// Error code and message to response
pub struct ErrorResponse {
    pub code: u16,
    pub msg: String,
}

// Convert ActionError to ResponseError
impl From<ActionError> for ErrorResponse {
    fn from(e: ActionError) -> Self {
        ErrorResponse {
            code: e.to_u16().unwrap(),
            msg: e.to_string(),
        }
    }
}

macro_rules! convert_error_type {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for ErrorResponse {
            fn from(e: $src_err_type) -> Self {
                Self {
                    code: 1,
                    msg: e.to_string(),
                }
            }
        }
    };
}

use reqwest::Error as ReqwestError;
convert_error_type!(ReqwestError);

use sled::Error as SledError;
convert_error_type!(SledError);

type E = Box<dyn std::error::Error>;
convert_error_type!(E);
