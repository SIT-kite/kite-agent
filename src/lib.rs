#![allow(dead_code)]
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

pub use error::Result;
pub use net::{Session, SessionStorage};
pub use service::portal_login;
