use std::fs::ReadDir;

use chrono::Utc;
use reqwest::header::{HeaderValue, COOKIE, USER_AGENT};
use reqwest::{Method, Response, StatusCode};

use crate::config::CONFIG;
use crate::config::USERAGENT;
use crate::error::Result;

use super::Session;

/// Get domain by url. The url must be started with `http://` or `https://` and a splash needed to
/// after the domain. The function used to get domain and pick cookies from cookie store by name, or
/// determine whether the path is relative or absolute.
pub fn domain(url: &str) -> Option<String> {
    let regex = regex::Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)(:[0-9]+)?/").unwrap();
    regex.captures(url).map(|x| x[1].to_string())
}

fn is_request_redirecting(status: reqwest::StatusCode) -> bool {
    status == StatusCode::MOVED_PERMANENTLY || status == StatusCode::PERMANENT_REDIRECT
}

#[inline]
pub async fn send_request(
    mut session: Session,
    mut client: reqwest::Client,
    mut request: reqwest::Request,
) -> Result<Response> {
    let mut next_hop = request.url().to_string();
    let domain = domain(&next_hop).expect("Could not parse domain.");

    let mut max_req_count = 10;

    loop {
        // Query cookie store
        let cookies = session.get_cookie_string(&domain);
        if !cookies.is_empty() {
            request
                .headers_mut()
                .append("Cookies", HeaderValue::from_str(&cookies)?);
        }
        let mut request = request.try_clone().unwrap();
        *request.method_mut() = reqwest::Method::GET;
        let response = client.execute(request).await?;
        session.sync_cookies(&domain, response.cookies());

        if max_req_count <= 0 || !is_request_redirecting(response.status()) {
            return Ok(response);
        }
        // Now the status code is 301 or 302.
        if let Some(new_url) = response.headers().get("Location") {
            let url = new_url.to_str().unwrap();
            next_hop = if domain(url).is_none() {
                // Relative path
                format!("http://{}/{}", domain, url)
            } else {
                url.to_string()
            }
        } else {
            return Ok(response);
        }

        max_req_count -= 1;
    }
}
