use crate::error::Result;
use crate::parser::edu::{get_f32, get_str};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static::lazy_static! {
    static ref WEEK_REGEX: Regex = Regex::new(r"(\d{1,2})(:?-(\d{1,2}))?").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    /// 课程名称
    pub(crate) course_name: String,
    /// 星期
    day: i32,
    /// 节次
    time_index: Vec<String>,
    /// 周次
    weeks: Vec<String>,
    /// 教室
    place: String,
    /// 教师
    teacher: Vec<String>,
    /// 校区
    campus: String,
    /// 学分
    credit: f32,
    /// 学时
    hours: f32,
    /// 教学班
    dyn_class_id: String,
    /// 课程代码
    course_id: String,
    /// 陪课班
    prefered_class: Vec<String>,
}

fn trans_week(week_day: &str) -> i32 {
    match week_day {
        "星期一" => 1,
        "星期二" => 2,
        "星期三" => 3,
        "星期四" => 4,
        "星期五" => 5,
        "星期六" => 6,
        "星期日" => 7,
        _ => 0,
    }
}

pub fn expand_weeks_str(week_string: &str) -> Vec<String> {
    let check_time_index = |x: &str| -> i32 {
        if let Ok(x) = x.parse() {
            return x;
        }
        0
    };

    let transform_number = |x: i32| -> String { x.to_string() };

    let mut weeks = Vec::new();
    week_string.split(',').for_each(|week_string| {
        if week_string.contains('-') {
            let mut step = 1;
            if week_string.ends_with("(单)") || week_string.ends_with("(双)") {
                step = 2;
            }
            let range = WEEK_REGEX.captures(week_string).unwrap();
            let mut min = check_time_index(range.get(1).unwrap().as_str());
            let max = check_time_index(range.get(3).unwrap().as_str());
            while min < max + 1 {
                weeks.push(transform_number(min));
                min += step;
            }
        } else {
            weeks.push(week_string.replace("周", ""));
        }
    });

    weeks
}

pub fn expand_time_index(time_string: &str) -> Vec<String> {
    let check_time_index = |x: &str| -> i32 {
        if let Ok(x) = x.parse() {
            return x;
        }
        0
    };
    let transform_number = |x: i32| -> String { x.to_string() };

    let mut indices = Vec::new();
    if time_string.contains('-') {
        if let Some((min, max)) = time_string.split_once('-') {
            let (range_left, range_right) = (check_time_index(min), check_time_index(max));
            let ranges = range_left..(range_right + 1);
            for range in ranges {
                indices.push(transform_number(range));
            }
        }
    } else {
        indices.push(String::from(time_string));
    }
    indices
}

fn split_string(s: String) -> Vec<String> {
    let result: Vec<String> = s.split(',').map(ToString::to_string).collect();
    result
}

pub fn parse_timetable_page(page: &str) -> Result<Vec<Course>> {
    let json_page: Value = serde_json::from_str(page)?;
    let course_list = json_page["kbList"].clone();
    if let Some(course) = course_list.as_array() {
        let mut result = Vec::new();
        for each_course in course {
            result.push(Course {
                course_name: get_str(each_course.get("kcmc")),
                day: trans_week(get_str(each_course.get("xqjmc")).as_str()),
                time_index: expand_time_index(get_str(each_course.get("jcs")).as_str()),
                weeks: expand_weeks_str(get_str(each_course.get("zcd")).as_str()),
                place: get_str(each_course.get("cdmc")),
                teacher: split_string(get_str(each_course.get("xm"))),
                campus: get_str(each_course.get("xqmc")),
                credit: get_f32(each_course.get("xf")),
                hours: get_f32(each_course.get("zxs")),
                dyn_class_id: get_str(each_course.get("jxbmc")),
                course_id: get_str(each_course.get("kch")),
                prefered_class: split_string(get_str(each_course.get("jxbzc"))),
            })
        }
        return Ok(result);
    }
    Ok(vec![])
}
