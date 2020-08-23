#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

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
        let remote_server = CONFIG.server.websocket.clone();
        let storage = session_storage.clone();

        tokio::spawn(async move {
            let mut agent = AgentBuilder::new(local_name)
                .host(remote_server)
                .set_callback(on_new_request, storage)
                .build();

            agent.start().await;
        });
    }

    loop {
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}
