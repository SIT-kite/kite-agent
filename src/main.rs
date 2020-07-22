#[macro_use]
extern crate lazy_static;

use crate::error::Result;
use crate::network::test_network_connectivity;
use crate::parsers::{ActivityDetail, TryParse};

use std::io::Read;
use std::time::Instant;

mod error;
mod network;
mod parsers;
mod user_agent;

fn main() {
    let content = std::fs::read_to_string("html/第二课堂详情页面.html").unwrap();
    let now = Instant::now();
    let r: Result<ActivityDetail> = TryParse::try_from_html(content.as_ref());
    println!("{}", now.elapsed().as_millis());
    println!("{:?}", r);
}
