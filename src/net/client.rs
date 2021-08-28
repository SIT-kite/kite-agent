use reqwest::header::HeaderValue;
use reqwest::{Client, Response, StatusCode};

use crate::error::Result;

use super::Session;

/// Get domain by url. The url must be started with `http://` or `https://` and a splash needed to
/// after the domain. The function used to get domain and pick cookies from cookie store by name, or
/// determine whether the path is relative or absolute.
pub fn parse_domain(url: &str) -> Option<String> {
    let regex = regex::Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)(:[0-9]+)?/").unwrap();
    regex.captures(url).map(|x| x[1].to_string())
}

pub fn parse_protocol(url: &str) -> String {
    if url.starts_with("https") {
        return String::from("https");
    }
    String::from("http")
}

pub enum Action {
    Redirect(String),
    Done,
}

type RequestHook = fn(&mut reqwest::Request);
type ResponseHook = fn(&mut reqwest::Response) -> Action;

pub struct UserClient {
    pub session: Session,
    pub raw_client: Client,

    request_hook: Option<RequestHook>,
    response_hook: Option<ResponseHook>,
}

impl UserClient {
    pub fn new(session: Session, raw_client: &Client) -> UserClient {
        Self {
            session,
            raw_client: raw_client.clone(),
            request_hook: None,
            response_hook: None,
        }
    }

    pub fn set_request_hook(&mut self, hook: Option<RequestHook>) {
        self.request_hook = hook;
    }

    pub fn set_response_hook(&mut self, hook: Option<ResponseHook>) {
        self.response_hook = hook;
    }

    pub async fn send(&mut self, request: reqwest::Request) -> Result<Response> {
        let mut complete_url = String::new();
        let mut request = request;

        loop {
            /* Parse domain and load cookies from session */
            complete_url = request.url().to_string();

            let domain = parse_domain(&complete_url).expect("Could not parse domain.");
            let cookies = self.session.get_cookie_string(&domain);

            if !cookies.is_empty() {
                request
                    .headers_mut()
                    .append("cookie", HeaderValue::from_str(&cookies)?);
            }

            /* Call request hook */
            self.request_hook.map(|hook| hook(&mut request));
            /* Execute request */
            let mut response = self.raw_client.execute(request).await?;
            /* Store new cookies to session */
            self.session.sync_cookies(&domain, response.cookies());
            /* Call response hook */
            match self
                .response_hook
                .map(|hook| hook(&mut response))
                .unwrap_or(Action::Done)
            {
                Action::Redirect(next_hop) => {
                    complete_url = next_hop;
                    request = self.raw_client.get(&complete_url).build()?;
                }
                Action::Done => {
                    return Ok(response);
                }
            }
        }
        /* Unreachable. */
    }

    pub async fn login_with_session(&mut self) -> Result<()> {
        self.session.login(&self.raw_client).await
    }
}

pub fn is_request_redirecting(status: reqwest::StatusCode) -> bool {
    status == StatusCode::FOUND || status == StatusCode::MOVED_PERMANENTLY
}

pub fn default_response_hook(response: &mut Response) -> Action {
    let status = response.status();
    let old_url = response.url().to_string();

    if is_request_redirecting(status) {
        if let Some(new_url) = response.headers().get("Location") {
            let url = new_url.to_str().unwrap();
            // TODO: How about this: /example.com/index.html ?
            let next_hop = if parse_domain(url).is_none() {
                // Relative path
                let old_domain = parse_domain(&old_url).unwrap();
                let protocol = parse_protocol(&old_url);

                format!("{}://{}/{}", protocol, old_domain, url)
            } else {
                url.to_string()
            };
            return Action::Redirect(next_hop);
        }
    }
    Action::Done
}
