pub use edu::{
    calculate_gpa, get_score_detail, parse_available_course_page, parse_class_list_page,
    parse_major_list_page, parse_profile_page, parse_score_list_page, parse_timetable_page,
};
pub use edu::{Class, Course, Major, Profile, SchoolYear, Score, ScoreDetail, SelectCourse, Semester};
pub use expense::ExpenseRecord;
pub use sc::{
    get_my_activity_list, get_my_score_list, Activity, ActivityDetail, JoinedActivity, ScActivityItem,
    ScImages, ScScoreItem, ScScoreSummary,
};

pub use library::{HoldingPreviews, SearchLibraryResult};

use crate::error::Result;

mod edu;
mod expense;
mod library;
mod sc;

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
