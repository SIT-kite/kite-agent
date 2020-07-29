#[macro_use]
extern crate lazy_static;

use crate::error::Result;
use crate::network::test_network_connectivity;
use crate::parsers::{ActivityDetail, Parse, PlannedCourse, TryParse};

use scraper::{ElementRef, Html, Selector};
use std::io::Read;
use std::time::Instant;

mod agent;
mod error;
mod network;
mod parsers;
mod user_agent;

fn main() {}
