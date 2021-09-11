use reqwest::{Request, StatusCode};

use crate::error::Result;
use crate::make_parameter;
use crate::service::ActionError;

use super::client::is_request_redirecting;
use super::{Session, UserClient};

/// Login page.
#[allow(dead_code)]
const LOGIN_URL: &str = "https://authserver.sit.edu.cn/authserver/login";
#[allow(dead_code)]
const NEED_CAPTCHA_URL: &str = "https://authserver.sit.edu.cn/authserver/needCaptcha.html";
#[allow(dead_code)]
const CAPTCHA_URL: &str = "https://authserver.sit.edu.cn/authserver/captcha.html";

/// Search in text by regex, and return the first group.
#[macro_export]
macro_rules! regex_find {
    ($text: expr, $pattern: expr) => {{
        let re = regex::Regex::new($pattern).unwrap();
        re.captures($text).map(|r| r[1].to_string())
    }};
}

/// Check whether captcha is need or not.
pub async fn check_need_captcha(client: &mut UserClient, account: &str) -> Result<bool> {
    let url = format!(
        "{}?{}",
        NEED_CAPTCHA_URL,
        make_parameter!(
            "username" => account,
            "pwdEncrypt2" => "pwdEncryptSalt")
    );
    let request = Request::new(reqwest::Method::GET, url.parse()?);
    let check_result = client.send(request).await?;

    let result_text = check_result.text().await?;
    println!("result_text = {}", result_text);
    Ok(result_text == "true")
}

/// Fetch captcha image.
pub async fn fetch_image(client: &mut UserClient) -> Result<Vec<u8>> {
    let request = Request::new(reqwest::Method::GET, CAPTCHA_URL.parse()?);
    let captcha = client.send(request).await?;

    if captcha.status() != StatusCode::OK {
        return Err(ActionError::FailToGetCaptcha.into());
    }
    return Ok(captcha.bytes().await?.to_vec());
}

/// Strip and remove blanks in verify code
fn clean_verify_code(code: &str) -> String {
    code.chars()
        .filter(|ch| ch.is_ascii_alphabetic() || ch.is_ascii_digit())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

/// Identify captcha images
fn identify_captcha(image_content: Vec<u8>) -> Result<String> {
    use imageproc::contrast::threshold;

    let image = image::load_from_memory_with_format(&image_content, image::ImageFormat::Jpeg)?;
    let image_luma = image.into_luma();
    let dimension = image_luma.dimensions();

    threshold(&image_luma, 130);
    let content = image_luma.into_vec();
    let text = tesseract::ocr_from_frame(
        &content,
        dimension.0 as i32,
        dimension.1 as i32,
        1,
        dimension.0 as i32,
        "num",
    )?;

    Ok(clean_verify_code(&text))
}

/// Login on campus official auth-server with student id and password.
/// Return string of cookies on `.sit.edu.cn`.
pub async fn portal_login(
    raw_client: &reqwest::Client,
    user_name: &str,
    password: &str,
) -> Result<Session> {
    let mut try_count = 8;

    let session = Session::new(user_name, password);
    let mut client = UserClient::new(session, raw_client);

    while try_count > 0 {
        client.session.cookies.clear();

        // Request login page to get encrypt key and so on.
        let index_request = Request::new(reqwest::Method::GET, LOGIN_URL.parse()?);
        let index_response = client.send(index_request).await?;
        let index_html = index_response.text().await?;
        let aes_key = regex_find!(&index_html, r#"var pwdDefaultEncryptSalt = "(.*?)";"#).unwrap();

        let need_captcha = check_need_captcha(&mut client, user_name).await?;
        let mut captcha = String::new();
        if need_captcha {
            loop {
                let image = fetch_image(&mut client).await?;
                captcha = identify_captcha(image)?;
                // Captcha code must be 4 chars. Continue if not.
                if captcha.len() == 4 {
                    break;
                }
            }
        }
        let login_request = client
            .raw_client
            .post(LOGIN_URL)
            .form(&[
                ("username", user_name),
                (
                    "password",
                    &generate_password_string(&password.to_string(), &aes_key),
                ),
                ("dllt", "userNamePasswordLogin"),
                ("execution", "e1s1"),
                ("_eventId", "submit"),
                ("rmShown", "1"),
                ("captchaResponse", &captcha),
                (
                    "lt",
                    &regex_find!(&index_html, r#"<input type="hidden" name="lt" value="(.*?)"/>"#)
                        .unwrap(),
                ),
            ])
            .build()?;

        let response = client.send(login_request).await?;
        // Login successfully.
        if is_request_redirecting(response.status()) {
            return Ok(client.session);
        }
        // Password error
        if response.status() == StatusCode::OK {
            let response_text = response.text().await?;
            if response_text.contains("您提供的用户名或者密码有误") {
                // If successfully authenticated or password wrong, break.
                return Err(ActionError::LoginFailed.into());
            } else {
                // Else, captcha wrong, or other error, make a captcha challenge again.
            }
        }

        try_count -= 1;
    }
    Err(ActionError::Unknown.into())
}

/// When submit password to `authserver.sit.edu.cn`, it's required to do AES and base64 algorithm with
/// origin password. We use a key from HTML (generated and changed by `JSESSIONID`) to help with.
pub fn generate_password_string(clear_password: &str, key: &str) -> String {
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
    let encrypted_password = cipher.encrypt_vec(&content);
    base64::encode(encrypted_password)
}
