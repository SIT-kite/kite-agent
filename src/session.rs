// use crate::error::Result;
// use chrono::{NaiveDateTime, Utc};
// use std::cell::Cell;
// use std::collections::HashMap;
//
// /// Db filename.
// const DB_FILE: &str = "kite-cache";
//
// lazy_static! {
//     pub static ref session_storage: Cell<SessionStorage> = Cell::new(SessionStorage::new());
// }
//
// /// Session structure key format in relation.
// const SESSION_KEY_FORMAT: &str = "s:{}";
//
// pub enum SessionError {}
//
// struct SessionStorage {
//     /// Sled handle
//     db: sled::Db,
// }
//
// impl SessionStorage {
//     /// Create a session storage client.
//     pub fn new() -> Result<Self> {
//         let db = sled::open(DB_FILE)?;
//         Ok(Self { db })
//     }
//
//     /// Query session by user.
//     pub fn query(&mut self, account: &str) -> Result<Option<Session>> {
//         // Query session struct from db.
//         let value_option = self.db.get(String::from(SESSION_KEY_FORMAT) + account)?;
//         if let Some(session_value) = value_option {
//             let session: Session = bincode::deserialize(session_value.as_ref())?;
//
//             return Ok(Some(session));
//         }
//         Ok(None)
//     }
//
//     /// Insert or update session data.
//     pub fn insert(&mut self, session: &Session) -> Result<()> {
//         let db_key = String::from(SESSION_KEY_FORMAT) + &session.account;
//         let value = bincode::serialize(session)?;
//
//         self.db.insert(&db_key, value)?;
//         Ok(())
//     }
// }
//
// /// Campus account login session
// pub struct Session {
//     /// Student ldap account
//     account: String,
//     /// Ldap raw password
//     password: String,
//     /// Http cookie, indexed by domains.
//     cookie: HashMap<String, String>,
//     /// Last use time.
//     last_update: NaiveDateTime,
// }
//
// impl Session {
//     pub fn new(account: &str, password: Option<&str>) -> Result<Option<Self>> {
//         if password.is_none() {
//             return Ok(session_storage.query(account)?);
//         }
//         Ok(None)
//     }
//
//     pub async fn validate(&self) -> bool {
//         true
//     }
//
//     pub async fn refresh(&mut self) {}
//
//     pub async fn get() {}
// }
