mod course;
mod second_course;

pub use course::CourseScore;
pub use second_course::{Activity, JoinedActivity};

pub trait Parser {
    fn from_html(html_page: &str) -> Self;
}
