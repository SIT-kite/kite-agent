use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use reqwest::cookie::Cookie;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::error::Result;

/// Session structure key format in relation.
const SESSION_KEY_FORMAT: &str = "s:";

pub enum SessionError {}

#[derive(Debug, Clone)]
pub struct SessionStorage {
    /// Sled handle
    db: sled::Db,
    rng: rand::rngs::SmallRng,
}

impl SessionStorage {
    /// Create a session database client.
    pub fn new() -> Result<Self> {
        use rand::SeedableRng;

        let db = sled::Config::new()
            .mode(sled::Mode::HighThroughput)
            .path(&CONFIG.agent.db)
            .open()?;
        // Note: get rand seed is a high cost operation, so we share it in session storage.
        let os_rng = rand::rngs::OsRng::default();
        let rng = rand::rngs::SmallRng::from_rng(os_rng)?;

        Ok(Self { db, rng })
    }

    /// Query session by user.
    pub fn query(&self, account: &str) -> Result<Option<Session>> {
        // Query session struct from db.
        let value_option = self.db.get(String::from(SESSION_KEY_FORMAT) + account)?;

        if let Some(session_value) = value_option {
            let session: Session = bincode::deserialize::<Session>(session_value.as_ref())?;
            return Ok(Some(session));
        }
        Ok(None)
    }

    /// Insert or update session data.
    pub fn insert(&mut self, session: &Session) -> Result<()> {
        let db_key = String::from(SESSION_KEY_FORMAT) + &session.account;
        let value = bincode::serialize(session)?;

        self.db.insert(&db_key, value)?;
        Ok(())
    }

    /// List session
    pub fn list(&self, index: u16, size: u16) -> Result<Vec<Session>> {
        let sessions = self
            .db
            .iter()
            .skip((index * size) as usize)
            .take(size as usize)
            .filter_map(|item| {
                if let Ok((_, value)) = item {
                    bincode::deserialize::<Session>(&value).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<Session>>();
        Ok(sessions)
    }
    /// Choose a session data randomly.
    pub fn choose_randomly(&mut self) -> Result<Option<Session>> {
        use rand::prelude::IteratorRandom;

        if let Some(Ok((_, session))) = self.db.iter().choose(&mut self.rng) {
            let content = session.to_vec();
            let session = bincode::deserialize::<Session>(&content)?;

            return Ok(Some(session));
        }
        Ok(None)
    }

    pub fn clear(&mut self) -> Result<()> {
        self.db.clear()?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}

// Note: You should not implement Default for SessionStorage. If you write code like this:
// SharedData {
//     parameter: SessionStorage,
//     ..SharedData::default(),
// }
// The default function will open database file separately, which may lead to:
// `The process cannot access the file because another process has locked a portion of the file.`
//
// impl Default for SessionStorage {
//     fn default() -> Self {
//         Self::new().unwrap()
//     }
// }

pub type AccountCookies = HashMap<String, HashMap<String, String>>;

/// Campus account login session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Student ldap account
    pub account: String,
    /// Ldap raw password
    pub password: String,
    /// Http cookie, indexed by domains.
    /// Stored in (domain, (key, value))
    pub cookies: AccountCookies,
    /// Last use time.
    pub last_update: NaiveDateTime,
}

impl Session {
    pub fn new(account: &str, password: &str) -> Self {
        Self {
            account: account.to_string(),
            password: password.to_string(),
            cookies: HashMap::default(),
            last_update: Utc::now().naive_utc(),
        }
    }

    // TODO: validate cookie.
    pub async fn validate(&self) -> Result<bool> {
        // use crate::service;
        //
        // service::portal_login(&self.account, &self.account).await?;
        Ok(true)
    }

    pub async fn login(&mut self, client: &reqwest::Client) -> Result<()> {
        self.cookies.clear();
        self.cookies = crate::service::portal_login(client, &self.account, &self.password)
            .await?
            .cookies;
        self.last_update = Utc::now().naive_local();

        Ok(())
    }

    pub fn get_cookie_string(&self, domain: &str) -> String {
        // TODO: If more than one domain has cookie with the same name, we may not know what we pick.
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
}

impl PartialEq<Session> for Session {
    fn eq(&self, other: &Session) -> bool {
        self.account == other.account && self.password == other.password && self.cookies == other.cookies
    }
}
