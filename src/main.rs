#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

mod actions;
mod communication;
mod error;
mod net;
mod parsers;

use communication::on_new_request;
use communication::{AgentBuilder, AgentData};
use tokio::time::Duration;

#[actix_rt::main]
async fn main() {
    let mut agent = AgentBuilder::new("0001".to_string())
        .host("wss://localhost.sunnysab.cn:8443/agent/")
        .set_callback(on_new_request, AgentData::new(String::new()))
        .build();

    agent.start().await;

    loop {
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}
