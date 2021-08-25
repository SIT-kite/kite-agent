use crate::net::AccountCookies;
use crate::zftool::client::ZfClient;
use crate::zftool::config::*;
use crate::zftool::error::{Result, ZfError};
use base64::{decode, encode};
use rand::rngs::OsRng;
use regex::Regex;
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::{cookie::Cookie, Client, ClientBuilder, Response, StatusCode};
use rsa::{BigUint, PaddingScheme, PublicKey, RsaPublicKey};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref CSRF_TOKEN_REGEX: Regex = Regex::new(
            "<input type=\"hidden\" id=\"csrftoken\" name=\"csrftoken\" value=\"(.*)\"/>",
        ).expect("Invalid CSRF_TOKEN_REGEX");
    static ref DOMAIN_REGEX: Regex = Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)(:[0-9]+)?/").unwrap();
}

#[derive(Debug, Clone)]
pub struct ZfSession {
    pub(crate) user: String,
    pub(crate) passwd: String,
    pub(crate) client: Client,
    pub(crate) login_flag: bool,
    pub(crate) cookies: AccountCookies,
}

#[derive(Default)]
pub struct SessionBuilder {
    user: Option<String>,
    passwd: Option<String>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        SessionBuilder::default()
    }

    pub fn user<T: ToString>(mut self, user: T) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn passwd<T: ToString>(mut self, passwd: T) -> Self {
        self.passwd = Some(passwd.to_string());
        self
    }

    pub fn build(self) -> ZfSession {
        ZfSession {
            user: self.user.unwrap_or_else(|| {
                panic!("User is required in SessionBuilder, please call user method.")
            }),
            passwd: self.passwd.unwrap_or_else(|| {
                panic!("Passwd is required in SessionBuilder, please call passwd method.")
            }),
            client: ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            login_flag: false,
            cookies: HashMap::default(),
        }
    }
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

impl ZfSession {
    async fn post_with_auto_redirect(
        &mut self,
        start_page: &str,
        params: [(&str, &str); 4],
    ) -> anyhow::Result<Response> {
        let mut remain_redirect = 10;
        let mut next_hop = start_page.to_string();

        let mut response = self
            .client
            .post(&next_hop)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.get_cookie_string("jwxt.sit.edu.cn"))
            .form(&params)
            .send()
            .await?;
        self.sync_cookies("jwxt.sit.edu.cn", response.cookies());

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
            response = self
                .client
                .get(&next_hop)
                .header(USER_AGENT, USERAGENT)
                .header(COOKIE, self.get_cookie_string("jwxt.sit.edu.cn"))
                .send()
                .await?;
            self.sync_cookies("jwxt.sit.edu.cn", response.cookies());
            remain_redirect -= 1;
        }
        Ok(response)
    }

    // Cookies function
    pub fn get_cookie_string(&self, domain: &str) -> String {
        let mut cookie_pairs = HashMap::<String, String>::new();
        self.cookies.iter().for_each(|(key, value)| {
            if domain.ends_with(key) {
                for (v0, v1) in value {
                    cookie_pairs.insert(v0.clone(), v1.clone());
                }
            }
        });
        cookie_pairs
            .into_iter()
            .map(|(k, v)| format!("{}={};", k, v))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn query_cookie(&self, domain: &str, name: &str) -> Option<&String> {
        for (key, domain_cookies) in self.cookies.iter() {
            if domain.ends_with(key) {
                if let Some(value) = domain_cookies.get(name) {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn sync_cookies<'a, T>(&mut self, domain: &str, cookies: T)
    where
        T: Iterator<Item = Cookie<'a>>,
    {
        cookies.for_each(|x| {
            let domain = x.domain().unwrap_or(domain);
            let mut domain_cookies = if let Some(c) = self.cookies.remove(domain) {
                c
            } else {
                HashMap::new()
            };
            domain_cookies.insert(x.name().to_string(), x.value().to_string());
            self.cookies.insert(String::from(domain), domain_cookies);
        });
    }

    // Passwd ras function
    pub async fn get_ras_public_key(&mut self) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
        #[derive(Debug, serde::Deserialize)]
        struct RsaPublicKey {
            modulus: String,
            exponent: String,
        }

        let resp = self
            .client
            .get(url::RSA_PUBLIC_KEY)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;

        self.sync_cookies("jwxt.sit.edu.cn", resp.cookies());
        let public_key = resp.json::<RsaPublicKey>().await?;
        let modulus = decode(public_key.modulus)?;
        let exponent = decode(public_key.exponent)?;
        Ok((modulus, exponent))
    }

    pub fn get_csrf_token(&self, login_page: &str) -> anyhow::Result<String> {
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

    // Login function
    pub async fn login(&mut self) -> Result<ZfClient> {
        // Get login page for the first cookie
        self.cookies.clear();

        let login_page = self
            .client
            .get(url::HOME)
            .header(USER_AGENT, USERAGENT)
            .send()
            .await?;
        self.sync_cookies("jwxt.sit.edu.cn", login_page.cookies());

        let text = login_page.text().await?;
        let token = self.get_csrf_token(&text)?;

        if let Ok((public_key, exponent)) = self.get_ras_public_key().await {
            let encrypted_passwd = encrypt_in_rsa(self.passwd.as_bytes(), public_key, exponent)?;

            let params = [
                ("csrftoken", token.as_str()),
                ("language", "zh_CN"),
                ("yhm", &self.user.clone()),
                ("mm", &encrypted_passwd),
            ];

            let final_response = self.post_with_auto_redirect(url::LOGIN, params).await?;
            return if final_response.url().to_string().starts_with(url::LOGIN) {
                let text = final_response.text().await?;
                let error = Self::parse_err_message(&text);

                Err(ZfError::SessionError(error).into())
            } else {
                self.login_flag = true;
                Ok(ZfClient {
                    user: self.user.clone(),
                    session: self.clone(),
                })
            };
        }
        Err(ZfError::PublicKeyError.into())
    }
}
