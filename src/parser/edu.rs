use serde::{Deserialize, Deserializer, Serialize};

pub use classes::{parse_class_list_page, parse_major_list_page};
pub use classes::{Class, Major};
pub use profile::parse_profile_page;
pub use profile::Profile;
pub use score::Score;
pub use score::{calculate_gpa, parse_score_list_page};
pub use score_detail::{get_score_detail, ScoreDetail};
pub use select_course::parse_available_course_page;
pub use select_course::SelectCourse;
pub use timetable::parse_timetable_page;
pub use timetable::Course;

use crate::parser::ParserError;

mod classes;
mod profile;
mod score;
mod score_detail;
mod select_course;
mod timetable;

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

    fn to_show(&self) -> i32 {
        match self {
            Semester::All => 0,
            Semester::FirstTerm => 1,
            Semester::SecondTerm => 2,
            Semester::MidTerm => 3,
        }
    }
}

pub fn vec_to_i32(s: Vec<i32>) -> i32 {
    let mut binary_number = 0;
    for number in s {
        binary_number |= 1 << number;
    }
    binary_number
}

pub fn str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let i = s.parse::<f32>().unwrap_or(-1.0);
    Ok(i)
}

pub fn str_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let i = s.parse::<i32>().unwrap_or_default();
    Ok(i)
}

pub fn str_to_semester<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let i = Semester::from_raw(&s).unwrap();
    let r = i.to_show();
    Ok(r)
}

pub fn str_to_none() -> Vec<String> {
    let mut s = Vec::new();
    s.push(String::from("空"));
    s
}
