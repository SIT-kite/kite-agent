#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

// Jemallocator support.
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod communication;
mod config;
mod error;
mod net;
mod parser;
mod service;

use crate::net::SessionStorage;
use communication::on_new_request;
use communication::AgentBuilder;
use config::CONFIG;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let session_storage = SessionStorage::new().unwrap();

    for _ in 0..CONFIG.server.conn {
        let local_name = CONFIG.agent.name.clone();
        let remote_server = CONFIG.server.addr.clone();
        let storage = session_storage.clone();

        tokio::spawn(async move {
            loop {
                let mut agent = AgentBuilder::new(local_name.clone())
                    .host(remote_server.clone())
                    .set_callback(on_new_request, storage.clone())
                    .build();

                match agent.start().await {
                    Ok(_) => {
                        agent.join().await;
                        println!("The host disconnected.");
                    }
                    Err(e) => println!("Could not connect to the host. Wait for the next try."),
                }
                println!("Wait for 30 secs.");
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
    }

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
