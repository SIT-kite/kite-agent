mod auth;
mod bill;

pub use auth::portal_login;

pub use bill::ElectricityBillRequest;

#[derive(Debug, thiserror::Error, ToPrimitive)]
pub enum ActionError {
    #[error("用户名或密码错误")]
    LoginFailed = 50,
}

/// Concat parameters to a url-formed string.
#[macro_export]
macro_rules! make_parameter {
    // Concatenate web form parameters to a string.
    ($($para: expr => $val: expr), *) => {{
        let mut url = String::new();
        $( url = url + $para + "=" + $val + "&"; )*

        url.clone()
    }}
}
