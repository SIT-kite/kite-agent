use crate::error::Result;
use crate::make_parameter;
use crate::service::ActionError;
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

/// Login page.
#[allow(dead_code)]
const LOGIN_URL: &str = "https://authserver.sit.edu.cn/authserver/login";

/// Search in text by regex, and return the first group.
#[macro_export]
macro_rules! regex_find {
    ($text: expr, $pattern: expr) => {{
        let re = regex::Regex::new($pattern).unwrap();
        re.captures($text).map(|r| r[1].to_string())
    }};
}

/// Login on campus official auth-server with student id and password.
/// Return string of cookies on `.sit.edu.cn`.
pub async fn portal_login(user_name: &str, password: &str) -> Result<HashMap<String, String>> {
    let client = Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()?;

    // Request login page to get encrypt key and so on.
    let mut first_response = client.get(LOGIN_URL).send().await?;
    let index_html = first_response.text().await?;

    let aes_key = regex_find!(&index_html, r#"var pwdDefaultEncryptSalt = "(.*?)";"#).unwrap();
    let response = client
        .post(LOGIN_URL)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(make_parameter!(
            "username" => user_name,
            "password" => &urlencoding::encode(&generate_passwd_string(&password.to_string(), &aes_key)),
            "dllt" => "userNamePasswordLogin",
            "execution" => "e1s1",
            "_eventId" => "submit",
            "rmShown" => "1",
            "lt" => &regex_find!(&index_html, r#"<input type="hidden" name="lt" value="(.*?)"/>"#).unwrap()
        ))
        .send()
        .await?;

    if response.status() == StatusCode::FOUND {
        let mut results = HashMap::new();
        // Default domain (or host) is where we request.
        let default_domain = "authserver.sit.edu.cn";

        for x in response.cookies() {
            // If the response set cookie in a given domain
            // For example, authserver may set cookies on .sit.edu.cn
            let current_domain = x.domain().unwrap_or(default_domain).to_string();
            let mut val = if let Some(v) = results.remove(&current_domain) {
                v
            } else {
                String::new()
            };
            val.push_str(&format!("{}={}; ", x.name(), urlencoding::encode(x.value())));
            results.insert(current_domain, val);
        }
        return Ok(results);
    }
    Err(ActionError::LoginFailed.into())
}

/// When submit password to `authserver.sit.edu.cn`, it's required to do AES and base64 algorithm with
/// origin password. We use a key from HTML (generated and changed by `JSESSIONID`) to help with.
pub fn generate_passwd_string(clear_password: &String, key: &String) -> String {
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    type Aes128Cbc = Cbc<aes::Aes128, Pkcs7>;

    // Create an AES object.
    let cipher = Aes128Cbc::new_var(key.as_bytes(), &[0u8; 16]).unwrap();
    // Concat plaintext: 64 bytes random bytes and original password.
    let mut content = Vec::new();
    content.extend_from_slice(&[0u8; 64]);
    content.extend_from_slice(clear_password.as_bytes());

    // Encrypt with AES and use do base64 encoding.
    let encrypted_passwd = cipher.encrypt_vec(&content);
    base64::encode(encrypted_passwd)
}
