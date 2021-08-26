mod edu;
mod expense;
mod second_course;

use crate::error::Result;

pub use edu::{
    calculate_gpa, parse_available_course_page, parse_class_list_page, parse_major_list_page,
    parse_profile_page, parse_score_list_page, parse_timetable_page,
};
pub use edu::{Class, Course, Major, Profile, SchoolYear, Score, SelectCourse, Semester};
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
    #[error("Profile element is wrong!!")]
    MissingField,
    #[error("Invalid semester valid given.")]
    SemesterError,
}
