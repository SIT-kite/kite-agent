use super::Session;
use crate::config::CONFIG;
use crate::error::Result;
use chrono::Utc;

/// Get domain by url. The url must be started with `http://` or `https://` and a splash needed to
/// after the domain. The function used to get domain and pick cookies from cookie store by name, or
/// determine whether the path is relative or absolute.
pub fn domain(url: &str) -> Option<String> {
    let regex = regex::Regex::new(r"http[s]?://([a-zA-Z\.0-9]+)(:[0-9]+)?/").unwrap();
    regex.captures(url).map(|x| x[1].to_string())
}

/// Create a client with config
pub struct ClientBuilder {
    /// User config, session store
    session: Session,
    /// Reqwest client builder
    client_builder: reqwest::ClientBuilder,
}

impl ClientBuilder {
    /// Create a client builder with user session
    pub fn new(session: Session) -> Self {
        Self {
            session,
            client_builder: reqwest::ClientBuilder::new(),
        }
    }

    /// Allow auto redirect.
    pub fn redirect(mut self, auto_redirect: bool) -> Self {
        if !auto_redirect {
            self.client_builder = self.client_builder.redirect(reqwest::redirect::Policy::none());
        }
        self
    }

    /// Set proxy for http and https
    pub fn proxy(mut self, proxy: &str) -> Self {
        if !proxy.is_empty() {
            self.client_builder = self.client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }
        self
    }

    /// Validate and build a client
    pub fn build(mut self) -> Client {
        // If global proxy is configured, set the global proxy
        // Note: add new proxy will push it to a proxy chain managed by reqwest library, and the global
        // proxy will be the last proxy in that chain.
        if let Some(proxy_string) = &CONFIG.agent.proxy {
            self = self.proxy(proxy_string.as_str());
        }

        let client = self.client_builder.build().unwrap();
        Client {
            session: self.session,
            client,
        }
    }
}

/// Http(s) Client, with cookie store.
pub struct Client {
    /// User config, session store
    session: Session,
    client: reqwest::Client,
}

impl Client {
    /// Create a get request
    pub fn get(&mut self, url: &str) -> RequestBuilder {
        RequestBuilder {
            session: &mut self.session,
            domain: domain(url).unwrap(),
            request_builder: self.client.get(url),
            payload: Vec::new(),
        }
    }

    /// Create a get request
    pub fn post(&mut self, url: &str) -> RequestBuilder {
        RequestBuilder {
            session: &mut self.session,
            domain: domain(url).unwrap(),
            request_builder: self.client.post(url),
            payload: Vec::new(),
        }
    }

    /// Get session reference
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Get session's mut reference
    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

pub struct RequestBuilder<'a> {
    /// User account and session
    session: &'a mut Session,
    /// Current request domain
    domain: String,
    /// Reqwest request builder
    request_builder: reqwest::RequestBuilder,
    /// Text payload
    payload: Vec<u8>,
}

impl<'a> RequestBuilder<'a> {
    /// Set request header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request_builder = self.request_builder.header(key, value);
        self
    }

    /// Set the form as the payload
    pub fn form(mut self, text: &'a str) -> Self {
        self.payload = text.as_bytes().to_vec();
        self.header("content-type", "application/x-www-form-urlencoded")
    }

    /// Set a text as the payload
    pub fn text(mut self, text: &'a str) -> Self {
        self.payload = text.as_bytes().to_vec();
        self
    }

    /// Set binary payload
    pub fn binary(mut self, content: &[u8]) -> Self {
        self.payload = content.to_vec();
        self
    }

    /// Send request
    pub async fn send(mut self) -> Result<reqwest::Response> {
        if !self.payload.is_empty() {
            // Note: performance issue.
            self.request_builder = self.request_builder.body(self.payload.to_vec());
        }

        // If cookie store is not empty, add cookie string in header.
        if !self.session.cookies.is_empty() {
            let cookie_str = self.session.get_cookie_string(&self.domain);
            self = self.header("cookie", &cookie_str);
        }

        // Use reqwest send the request
        let response = self.request_builder.send().await?;
        // Update cookies in session. Use Client::session to acquire.
        self.session.sync_cookies(&self.domain, response.cookies());
        self.session.last_update = Utc::now().naive_local();

        Ok(response)
    }
}
