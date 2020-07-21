#[macro_use]
extern crate lazy_static;

use crate::network::test_network_connectivity;
use crate::parsers::JoinedActivity;
use crate::parsers::Parser;

use std::io::Read;

mod error;
mod network;
mod parsers;
mod user_agent;

// #[actix_rt::main]
// async fn async_main() {
//     print!("{:?}", test_network_connectivity().await)
// }

fn main() {
    let file = std::fs::read_to_string("html/第二课堂得分页面.html").unwrap();
    let activities: Vec<JoinedActivity> = Parser::from_html(file.as_ref());

    println!("{:#?}", activities);
}
