mod bill;
mod course;
mod expense;
mod second_course;

use crate::error::{CrawlerError, Result};
use regex::Error as RegexError;
use thiserror::Error;

pub use bill::ElectricityBill;
pub use course::{CourseDetail, CourseScore, PlannedCourse, SelectedCourse};
pub use expense::ExpenseRecord;
pub use second_course::{Activity, ActivityDetail, JoinedActivity, SecondScore};

pub trait Parse {
    fn from_html(html_page: &str) -> Self;
}

pub trait TryParse {
    fn try_from_html(html_page: &str) -> Result<Self>
    where
        Self: std::marker::Sized;
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("找不到对应元素: {0}")]
    NoSuchElement(String),
    #[error("正则解析错误: {0}")]
    RegexErr(String),
}

impl From<RegexError> for ParserError {
    fn from(regex_err: RegexError) -> Self {
        ParserError::RegexErr(regex_err.to_string())
    }
}

impl From<ParserError> for CrawlerError {
    fn from(parser_err: ParserError) -> Self {
        CrawlerError::HtmlParser(parser_err.to_string())
    }
}

impl From<regex::Error> for CrawlerError {
    fn from(regex_err: regex::Error) -> Self {
        ParserError::RegexErr(regex_err.to_string()).into()
    }
}
