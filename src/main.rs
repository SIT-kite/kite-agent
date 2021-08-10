#![allow(dead_code)]

// Jemallocator support.
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

mod agent;
mod config;
mod error;
mod net;
mod parser;
pub mod service;

use agent::{run, SharedData};
use config::CONFIG;
use net::SessionStorage;

#[tokio::main]
async fn main() {
    let storage = SessionStorage::new().unwrap();

    let remote_server = &CONFIG.server.addr;
    let node_name = &CONFIG.agent.name;

    run(
        remote_server.clone(),
        SharedData {
            node: node_name.clone(),
            session: storage,
        },
    )
    .await;
}
