mod course;

pub use course::CourseScore;

pub trait Parser {
    fn from_html(html_page: &str) -> Self;
}
