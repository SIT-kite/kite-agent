#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

mod communication;
mod error;
mod net;
mod parser;
mod service;

pub use error::{AgentError, Result};
pub use net::{Session, SessionStorage};
pub use service::portal_login;
