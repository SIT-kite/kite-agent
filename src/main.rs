#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod actions;
mod agent;
mod error;
mod network;
mod parsers;
mod user_agent;

use crate::error::Result;
use agent::{AgentBuilder, Request, Response};
use tokio::time::Duration;

#[actix_rt::main]
async fn main() {
    let mut agent = AgentBuilder::new("0001".to_string())
        .host("wss://localhost.sunnysab.cn:8443/agent/")
        .set_callback(on_new_request, String::from("Hello world"))
        .build();

    agent.start().await;

    loop {
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}

// fn(Request, Data) -> crate::error::Result<Response>;
pub fn on_new_request(_request: Request, data: String) -> Result<Response> {
    println!("data = {}", data);
    Ok(Response)
}
