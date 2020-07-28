#[macro_use]
extern crate lazy_static;

use crate::error::Result;
use crate::network::test_network_connectivity;
use crate::parsers::{ActivityDetail, CoursePlan, Parse, TryParse};

use scraper::{ElementRef, Html, Selector};
use std::io::Read;
use std::time::Instant;

mod error;
mod network;
mod parsers;
mod user_agent;

fn main() {
    let html_page = std::fs::read_to_string("kite-crawler/html/教学计划查询页面.html").unwrap();

    let results: Vec<CoursePlan> = Parse::from_html(html_page.as_str());

    println!("{:#?}", results[1]);
}
