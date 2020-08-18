use chrono::Utc;
use kite_agent::{Session, SessionStorage};
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Insert, update, delete or show session(s)
pub enum SessionCommand {
    /// List sessions stored in the database in pagination.
    List(ListSession),
    /// Add a account to the database with account and credential
    Insert(InsertSession),
}

impl SessionCommand {
    pub async fn process(self, sessions: SessionStorage) {
        match self {
            SessionCommand::List(list) => list.process(sessions).await,
            SessionCommand::Insert(new) => new.process(sessions).await,
        }
    }
}

#[derive(StructOpt)]
/// Show sessions.
pub struct ListSession {
    #[structopt(long, short, default_value = "0")]
    index: u16,
    #[structopt(long, short, default_value = "10")]
    size: u16,
}

impl ListSession {
    pub async fn process(self, storage: SessionStorage) {
        let index = if self.index == 1 { 0 } else { self.index };
        let sessions = storage.list(index, self.size).unwrap();

        println!("{} result(s) found in page {}.", sessions.len(), self.index);
        let mut table = Table::new();

        table.add_row(row!["ACCOUNT", "CREDENTIAL", "LAST UPDATE"]);
        for session in sessions {
            table.add_row(Row::new(vec![
                Cell::new(&session.account),
                Cell::new(&session.password),
                Cell::new(&session.last_update.to_string()),
            ]));
        }
        table.printstd();
    }
}

#[derive(StructOpt)]
/// Insert a new session
pub struct InsertSession {
    #[structopt(long)]
    account: String,
    #[structopt(long)]
    credential: String,
}

impl InsertSession {
    pub async fn process(self, mut storage: SessionStorage) {
        println!("Connect and verify..");
        // Verify on authserver
        let r = kite_agent::portal_login(&self.account, &self.credential).await;

        match r {
            Ok(cookie) => {
                let mut cookie_map = HashMap::new();

                println!("Cookie in .sit.edu.cn: {}", cookie);
                cookie_map.insert(String::from(".sit.edu.cn"), cookie);

                let new_session = Session {
                    account: self.account,
                    password: self.credential,
                    cookie: cookie_map,
                    last_update: Utc::now().naive_local(),
                };
                println!("Write to database.");
                storage.insert(&new_session).unwrap();
            }
            Err(e) => println!("Failed to login: {:?}", e),
        }
    }
}
