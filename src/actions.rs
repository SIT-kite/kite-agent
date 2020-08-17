pub mod auth;

pub use auth::portal_login;

#[derive(Debug, thiserror::Error, ToPrimitive)]
pub enum ActionError {
    #[error("用户名或密码错误")]
    LoginFailed = 50,
}
