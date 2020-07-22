mod bill;
mod course;
mod second_course;

pub use bill::ElectricityBill;
pub use course::{CourseDetail, CourseScore};
pub use second_course::{Activity, JoinedActivity};

pub trait Parser {
    fn from_html(html_page: &str) -> Self;
}
