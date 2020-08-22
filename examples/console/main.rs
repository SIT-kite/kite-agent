#[macro_use]
extern crate prettytable;

/// Use the given account to fetch page
mod page;
/// Edit stored sessions
mod session;

use kite_agent::SessionStorage;
use std::error::Error;
use structopt::StructOpt;

use page::PageCommand;
use session::SessionCommand;

pub type ConsoleResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(StructOpt)]
#[structopt(name = "kite-agent-cli")]
enum Command {
    #[structopt(name = "session")]
    SessionCommand(SessionCommand),
    #[structopt(name = "page")]
    PageCommand(PageCommand),
}

#[tokio::main]
async fn main() {
    // Read commands from command line parameters.
    let command = Command::from_args();
    // Open session database.
    let session_storage = SessionStorage::new().unwrap();
    println!("Session storage opened.");

    let result = match command {
        Command::SessionCommand(c) => c.process(session_storage).await,
        Command::PageCommand(p) => p.process(session_storage).await,
    };

    match result {
        Ok(_) => (),
        Err(e) => {
            println!("Command did not execute successfully: {}", e.to_string());
        }
    }
}
