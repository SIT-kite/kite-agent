mod classes;
mod profile;
mod score;
mod select_course;
mod timetable;

pub use classes::{parse_class_list_page, parse_major_list_page};
pub use profile::parse_profile_page;
pub use score::{calculate_gpa, parse_score_list_page};
pub use select_course::parse_available_course_page;
pub use timetable::parse_timetable_page;

pub use classes::{Class, Major};
pub use profile::Profile;
pub use score::Score;
pub use select_course::SelectCourse;
pub use timetable::Course;

use crate::parser::ParserError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SchoolYear {
    AllYear,
    SomeYear(i32),
}

impl ToString for SchoolYear {
    fn to_string(&self) -> String {
        match self {
            SchoolYear::SomeYear(year) => year.to_string(),
            SchoolYear::AllYear => String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Semester {
    All = 0,
    FirstTerm = 1,
    SecondTerm = 2,
    MidTerm = 3,
}

impl Semester {
    pub(crate) fn to_raw(&self) -> &str {
        match self {
            Semester::All => "",
            Semester::FirstTerm => "3",
            Semester::SecondTerm => "12",
            Semester::MidTerm => "16",
        }
    }

    fn from_raw(raw: &str) -> Result<Semester, ParserError> {
        match raw {
            "" => Ok(Semester::All),
            "3" => Ok(Semester::FirstTerm),
            "12" => Ok(Semester::SecondTerm),
            "16" => Ok(Semester::MidTerm),
            _ => Err(ParserError::SemesterError),
        }
    }
}

pub fn get_str(x: Option<&Value>) -> String {
    String::from(x.map(|m| m.as_str().unwrap()).unwrap_or_default())
}

pub fn get_f32(x: Option<&Value>) -> f32 {
    get_str(x).parse().unwrap()
}
