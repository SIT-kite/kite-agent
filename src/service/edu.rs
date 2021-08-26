use crate::config::*;
use crate::error::{Result, ZfError};
use crate::net::Client;
use base64::{decode, encode};
use rand::rngs::OsRng;
use regex::Regex;
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::{Response, StatusCode};
use rsa::{BigUint, PaddingScheme, PublicKey, RsaPublicKey};

pub mod client;

lazy_static::lazy_static! {
    static ref CSRF_TOKEN_REGEX: Regex = Regex::new(
            "<input type=\"hidden\" id=\"csrftoken\" name=\"csrftoken\" value=\"(.*)\"/>",
        ).expect("Invalid CSRF_TOKEN_REGEX");
    static ref DOMAIN_REGEX: Regex = Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)(:[0-9]+)?/").unwrap();
}

pub fn encrypt_in_rsa(message: &[u8], public_key: Vec<u8>, exponent: Vec<u8>) -> anyhow::Result<String> {
    let key = BigUint::from_bytes_be(&public_key);
    let exp = BigUint::from_bytes_be(&exponent);
    let mut rng = OsRng;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let publickey = RsaPublicKey::new(key, exp)?;
    let enc_data = publickey
        .encrypt(&mut rng, padding, message)
        .expect("failed to encrypt");
    Ok(encode(enc_data))
}

pub fn domain(url: &str) -> Option<String> {
    DOMAIN_REGEX.captures(url).map(|x| x[1].to_string())
}

async fn post_with_auto_redirect(
    client: &mut Client,
    start_page: &str,
    params: &[(&str, &str)],
) -> anyhow::Result<Response> {
    let mut remain_redirect = 10i32;
    let mut next_hop = start_page.to_string();
    let session = client.session();

    let mut response = client
        .post(&next_hop)
        .header("User-Agent", USERAGENT)
        .header("Cookie", &session.get_cookie_string("jwxt.sit.edu.cn"))
        .form(&params)
        .send()
        .await?;

    while remain_redirect > 0 && response.status() == StatusCode::FOUND {
        let redirect_url = response.headers().get("location");
        if redirect_url.is_none() {
            return Ok(response);
        }
        let t = redirect_url.unwrap().to_str().unwrap().to_string();
        next_hop = if domain(&t).is_none() {
            format!("http://{}/{}", domain(&next_hop).unwrap(), t)
        } else {
            t
        };
        response = client
            .client
            .get(&next_hop)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        session.sync_cookies("jwxt.sit.edu.cn", response.cookies());
        remain_redirect -= 1;
    }
    Ok(response)
}

pub async fn get_rsa_public_key(client: Client) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    #[derive(Debug, serde::Deserialize)]
    struct RsaPublicKey {
        modulus: String,
        exponent: String,
    }
    let session = client.session();

    let resp = client
        .client
        .get(url::RSA_PUBLIC_KEY)
        .header(USER_AGENT, USERAGENT)
        .header(COOKIE, session.get_cookie_string("jwxt.sit.edu.cn"))
        .send()
        .await?;

    let public_key = resp.json::<RsaPublicKey>().await?;
    let modulus = decode(public_key.modulus)?;
    let exponent = decode(public_key.exponent)?;
    Ok((modulus, exponent))
}

pub fn get_csrf_token(login_page: &str) -> anyhow::Result<String> {
    let text = login_page;
    if let Some(token_tag) = CSRF_TOKEN_REGEX.captures(text) {
        let token = &token_tag[1];
        return Ok(token.to_string());
    }
    Ok(String::new())
}

fn parse_err_message(content: &str) -> String {
    use scraper::{Html, Selector};
    let document = Html::parse_document(content);
    let err_node: String = document
        .select(&Selector::parse("div#home.tab-pane.in.active p#tips.bg_danger.sl_danger").unwrap())
        .next()
        .unwrap()
        .text()
        .collect();
    err_node.trim().to_string()
}

pub async fn login(mut client: Client) -> Result<String> {
    // Get login page for the first cookie
    client.session.cookies.clear();

    let login_page = client
        .client
        .get(url::HOME)
        .header(USER_AGENT, USERAGENT)
        .send()
        .await?;
    client
        .session
        .sync_cookies("jwxt.sit.edu.cn", login_page.cookies());

    let text = login_page.text().await?;
    let token = get_csrf_token(&text)?;

    if let Ok((public_key, exponent)) = get_rsa_public_key(client).await {
        let encrypted_passwd = encrypt_in_rsa(client.session.password.as_bytes(), public_key, exponent)?;
        let params = [
            ("csrftoken", token.as_str()),
            ("language", "zh_CN"),
            ("yhm", &client.session.account.as_str()),
            ("mm", &encrypted_passwd),
        ];

        let final_response = post_with_auto_redirect(&mut client, url::LOGIN, &params).await?;
        return if final_response.url().to_string().starts_with(url::LOGIN) {
            let text = final_response.text().await?;
            let error = parse_err_message(&text);

            Err(ZfError::SessionError(error).into())
        } else {
            Ok(String::from("success"))
        };
    }
    Err(ZfError::PublicKeyError.into())
}
