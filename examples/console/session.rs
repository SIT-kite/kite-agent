use crate::ConsoleResult;
use kite_agent::SessionStorage;
use prettytable::{Cell, Row, Table};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Insert, update, delete or show session(s)
pub enum SessionCommand {
    /// List sessions stored in the database in pagination.
    List(ListSession),
    /// Add a account to the database with account and credential
    Insert(InsertSession),
    /// Delete all sessions
    Clean,
}

impl SessionCommand {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        match self {
            SessionCommand::List(list) => list.process(sessions).await,
            SessionCommand::Insert(new) => new.process(sessions).await,
            SessionCommand::Clean => Ok(CleanSession.process(sessions).await),
        }
    }
}

#[derive(StructOpt)]
/// Show sessions.
pub struct ListSession {
    #[structopt(long, short = "u")]
    account: Option<String>,
    #[structopt(long, short, default_value = "0")]
    index: u16,
    #[structopt(long, short, default_value = "10")]
    size: u16,
}

impl ListSession {
    pub fn print_account_list(storage: SessionStorage, page: u16, count: u16) -> ConsoleResult<()> {
        let index = if page == 1 { 0 } else { page };
        let sessions = storage.list(index, count)?;

        println!(
            "{} result(s) found in the page {}, total: {}",
            sessions.len(),
            page,
            storage.len()
        );
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
        Ok(())
    }

    pub fn print_cookie_list(storage: SessionStorage, account: String) -> ConsoleResult<()> {
        let session = storage.query(&account)?;
        let mut table = Table::new();

        if let Some(session) = session {
            table.add_row(row!["DOMAIN", "NAME", "VALUE"]);
            for each_domain in session.cookies {
                for each_cookie in each_domain.1 {
                    table.add_row(Row::new(vec![
                        Cell::new(&each_domain.0),
                        Cell::new(&each_cookie.0),
                        Cell::new(&each_cookie.1),
                    ]));
                }
            }
            table.printstd();
        } else {
            println!("Could not find account {}", account);
        }
        Ok(())
    }
    pub async fn process(self, storage: SessionStorage) -> ConsoleResult<()> {
        if let Some(account) = self.account {
            Self::print_cookie_list(storage, account);
        } else {
            Self::print_account_list(storage, self.index, self.size);
        }
        Ok(())
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
    pub async fn process(self, mut storage: SessionStorage) -> ConsoleResult<()> {
        println!("Connect and verify..");
        // Verify on authserver
        let session = kite_agent::portal_login(&self.account, &self.credential).await?;

        println!("Session {:#?}", session);
        println!("Write to database.");
        storage.insert(&session).unwrap();
        Ok(())
    }
}

/// Delete all sessions.
pub struct CleanSession;

impl CleanSession {
    pub async fn process(self, mut storage: SessionStorage) {
        storage.clear();
    }
}
