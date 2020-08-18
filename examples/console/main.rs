#[macro_use]
extern crate prettytable;

mod session;

use kite_agent::SessionStorage;
use session::SessionCommand;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kite-agent-cli")]
enum Command {
    #[structopt(name = "session")]
    SessionCommand(SessionCommand),
}

#[actix_rt::main]
async fn main() {
    // Read commands from command line parameters.
    let command = Command::from_args();
    // Open session database.
    let session_storage = SessionStorage::new().unwrap();
    println!("Session storage opened.");

    match command {
        Command::SessionCommand(c) => c.process(session_storage).await,
    }
    println!("Finished.")
}
