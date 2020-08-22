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

#[tokio::main]
async fn main() {

    // let session_storage = SessionStorage::new().unwrap();
    // let mut agent = AgentBuilder::new("0001".to_string())
    //     .host("wss://localhost.sunnysab.cn:8443/agent/")
    //     .set_callback(on_new_request, session_storage)
    //     .build();
    //
    // agent.start().await;
    //
    // loop {
    //     tokio::time::delay_for(Duration::from_secs(1)).await;
    // }
}
