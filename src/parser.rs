mod bill;
mod course;
mod expense;
mod second_course;

use crate::error::Result;

pub use bill::ElectricityBill;
pub use course::{CourseDetail, CourseScore, PlannedCourse, SelectedCourse};
pub use expense::ExpenseRecord;
pub use second_course::{Activity, ActivityDetail, JoinedActivity, SecondScore};

pub trait Parse {
    fn from_html(html_page: &str) -> Result<Self>
    where
        Self: std::marker::Sized;
}

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("找不到对应元素: {0}")]
    NoSuchElement(String),
    #[error("正则解析错误: {0}")]
    RegexErr(String),
}
