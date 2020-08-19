use crate::error::Result;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Db filename.
const DB_FILE: &str = "kite-cache";

/// Session structure key format in relation.
const SESSION_KEY_FORMAT: &str = "s:";

pub enum SessionError {}

#[derive(Clone)]
pub struct SessionStorage {
    /// Sled handle
    db: sled::Db,
}

impl SessionStorage {
    /// Create a session database client.
    pub fn new() -> Result<Self> {
        let db = sled::Config::new()
            .mode(sled::Mode::HighThroughput)
            .path(DB_FILE)
            .open()?;

        Ok(Self { db })
    }

    /// Query session by user.
    pub fn query(&mut self, account: &str) -> Result<Option<Session>> {
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
        // TODO: handle error
        if let Some(Ok((_, session))) = self.db.iter().next() {
            let content = session.to_vec();
            let session = bincode::deserialize::<Session>(&content)?;

            return Ok(Some(session));
        }
        Ok(None)
    }

    pub fn clear(&mut self) -> Result<()> {
        self.db.clear();
        Ok(())
    }
}

// Note: You should not implement Default for SessionStorage. If you write code like this:
// AgentData {
//     parameter: SessionStorage,
//     ..AgentData::default(),
// }
// The default function will open database file separately, which may lead to:
// `The process cannot access the file because another process has locked a portion of the file.`
//
// impl Default for SessionStorage {
//     fn default() -> Self {
//         Self::new().unwrap()
//     }
// }

/// Campus account login session
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// Student ldap account
    pub account: String,
    /// Ldap raw password
    pub password: String,
    /// Http cookie, indexed by domains.
    pub cookie: HashMap<String, String>,
    /// Last use time.
    pub last_update: NaiveDateTime,
}

impl Session {
    pub fn new(account: &str, password: &str) -> Self {
        Self {
            account: account.to_string(),
            password: password.to_string(),
            cookie: HashMap::default(),
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

    pub async fn login(&mut self) -> Result<()> {
        self.cookie.clear();
        self.cookie = crate::service::portal_login(&self.account, &self.account).await?;

        Ok(())
    }

    pub fn get_cookie_string(&self, domain: &str) -> String {
        let cookies = self
            .cookie
            .iter()
            .filter_map(|(key, value)| {
                if domain.ends_with(key) {
                    Some(format!("{}; ", value))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        cookies
    }
}
