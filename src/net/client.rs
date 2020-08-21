use super::Session;
use crate::error::Result;
use chrono::Utc;

pub fn domain(url: &str) -> Option<String> {
    let regex = regex::Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)/").unwrap();
    regex.captures(url).and_then(|x| Some(x[1].to_string()))
}

pub struct ClientBuilder {
    session: Session,
    client_builder: reqwest::ClientBuilder,
}

impl ClientBuilder {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            client_builder: reqwest::ClientBuilder::new(),
        }
    }

    pub fn redirect(mut self, auto_redirect: bool) -> Self {
        if !auto_redirect {
            self.client_builder = self.client_builder.redirect(reqwest::redirect::Policy::none());
        }
        self
    }

    pub fn proxy(mut self, proxy: &str) -> Self {
        if !proxy.is_empty() {
            self.client_builder = self.client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }
        self
    }

    pub fn build(self) -> Client {
        let client = self.client_builder.build().unwrap();

        Client {
            session: self.session,
            client,
        }
    }
}

pub struct Client {
    session: Session,
    client: reqwest::Client,
}

impl Client {
    pub fn get(&mut self, url: &str) -> RequestBuilder {
        RequestBuilder {
            session: &mut self.session,
            domain: domain(url).unwrap(),
            request_builder: self.client.get(url),
            payload: "",
        }
    }

    pub fn post(&mut self, url: &str) -> RequestBuilder {
        RequestBuilder {
            session: &mut self.session,
            domain: domain(url).unwrap(),
            request_builder: self.client.post(url),
            payload: "",
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

pub struct RequestBuilder<'a> {
    session: &'a mut Session,
    domain: String,
    request_builder: reqwest::RequestBuilder,
    payload: &'a str,
}

impl<'a> RequestBuilder<'a> {
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request_builder = self.request_builder.header(key, value);
        self
    }

    pub fn form(mut self, text: &'a str) -> Self {
        self.payload = text;
        self.header("content-type", "application/x-www-form-urlencoded")
    }

    pub fn text(mut self, text: &'a str) -> Self {
        self.payload = text;
        self
    }

    pub async fn send(mut self) -> Result<reqwest::Response> {
        if self.payload != "" {
            self.request_builder = self.request_builder.body(self.payload.to_string());
        }

        if !self.session.cookies.is_empty() {
            let cookie_str = self.session.get_cookie_string(&self.domain);
            self = self.header("cookie", &cookie_str);
        }

        let response = self.request_builder.send().await?;
        self.session.sync_cookies(&self.domain, response.cookies());
        self.session.last_update = Utc::now().naive_local();

        Ok(response)
    }
}
