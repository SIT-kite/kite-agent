use super::url;
use crate::error::{Result, ZfError};
use crate::net::UserClient;
use base64::{decode, encode};
use rand::rngs::OsRng;
use rsa::{BigUint, PaddingScheme, PublicKey, RsaPublicKey};
use scraper::{Html, Selector};
use std::ops::Deref;

lazy_static::lazy_static! {
    static ref CSRF_TOKEN_REGEX: regex::Regex = regex::Regex::new(
            "<input type=\"hidden\" id=\"csrftoken\" name=\"csrftoken\" value=\"(.*)\"/>",
        ).expect("Invalid CSRF_TOKEN_REGEX");
    static ref LOGIN_ERR_MSG_SELECTOR: Selector =
        Selector::parse("div#home.tab-pane.in.active p#tips.bg_danger.sl_danger")
        .expect("Invalid LOGIN_ERR_MSG_SELECTOR.");
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

pub async fn get_rsa_public_key(client: &mut UserClient) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    #[derive(Debug, serde::Deserialize)]
    struct RsaPublicKey {
        modulus: String,
        exponent: String,
    }

    let res = client.raw_client.get(url::RSA_PUBLIC_KEY).build()?;
    let resp = client.send(res).await?;

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
    let document = Html::parse_document(content);
    let err_node: String = document
        .select(LOGIN_ERR_MSG_SELECTOR.deref())
        .next()
        .unwrap()
        .text()
        .collect();
    err_node.trim().to_string()
}

pub async fn login(client: &mut UserClient) -> Result<String> {
    client.session.cookies.clear();

    let login = client.raw_client.get(url::HOME).build()?;
    let login_page = client.send(login).await?;

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
        let response_f = client.raw_client.post(url::LOGIN).form(&params).build()?;
        let final_response = client.send(response_f).await?;
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
