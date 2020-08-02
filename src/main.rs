#[macro_use]
extern crate lazy_static;

use crate::error::Result;
use crate::network::test_network_connectivity;
use crate::parsers::{ActivityDetail, ExpenseRecord, Parse, PlannedCourse, TryParse};

use error::CrawlerError;
use parsers::ParserError;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::io::Read;
use std::time::Instant;

mod error;
mod network;
mod parsers;
mod user_agent;

fn main() {
    let html_page = std::fs::read_to_string("kite-crawler/html/消费记录页面.html").unwrap();

    let res: Vec<ExpenseRecord> = Parse::from_html(html_page.as_str());

    println!("{:#?}", res)
}
