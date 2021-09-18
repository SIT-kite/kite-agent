use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::error::Result;
use crate::parser::edu::{str_to_f32, str_to_i32, vec_to_i32};

lazy_static::lazy_static! {
    static ref WEEK_REGEX: Regex = Regex::new(r"(\d{1,2})(:?-(\d{1,2}))?").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    #[serde(rename(deserialize = "kcmc"))]
    /// 课程名称
    course_name: String,
    #[serde(rename(deserialize = "xqjmc"), deserialize_with = "day_to_i32")]
    /// 星期
    day: i32,
    #[serde(rename(deserialize = "jcs"), deserialize_with = "index_time_to_i32")]
    /// 节次
    time_index: i32,
    #[serde(rename(deserialize = "zcd"), deserialize_with = "weeks_to_i32")]
    /// 周次
    week: i32,
    #[serde(rename(deserialize = "cdmc"))]
    /// 教室
    place: String,
    #[serde(rename(deserialize = "xm"), deserialize_with = "str_to_vec_string", default)]
    /// 教师
    teacher: Vec<String>,
    #[serde(rename(deserialize = "xqmc"))]
    /// 校区
    campus: String,
    #[serde(rename(deserialize = "xf"), deserialize_with = "str_to_f32")]
    /// 学分
    credit: f32,
    #[serde(rename(deserialize = "zxs"), deserialize_with = "str_to_i32")]
    /// 学时
    hours: i32,
    #[serde(rename(deserialize = "jxbmc"), deserialize_with = "str_trim")]
    /// 教学班
    dyn_class_id: String,
    #[serde(rename(deserialize = "kch"))]
    /// 课程代码
    course_id: String,
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

fn expand_weeks_collect(week_string: &str) -> Vec<i32> {
    let check_time_index = |x: &str| -> i32 {
        if let Ok(x) = x.parse() {
            return x;
        }
        0
    };

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
                weeks.push(min);
                min += step;
            }
        } else {
            weeks.push((week_string.replace("周", "")).parse().unwrap());
        }
    });

    weeks
}

fn expand_time_collect(time_string: &str) -> Vec<i32> {
    let check_time_index = |x: &str| -> i32 { x.parse().unwrap_or_default() };

    let mut indices = Vec::new();
    if time_string.contains('-') {
        if let Some((min, max)) = time_string.split_once('-') {
            let (range_left, range_right) = (check_time_index(min), check_time_index(max));
            let ranges = range_left..(range_right + 1);
            for range in ranges {
                indices.push(range);
            }
        }
    } else {
        indices.push(time_string.parse().unwrap());
    }
    indices
}

fn split_string(s: &str) -> Vec<String> {
    if s.is_empty() {
        Vec::new()
    } else {
        s.split(',').map(ToString::to_string).collect()
    }
}

fn weeks_to_i32<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let x = expand_weeks_collect(&s);
    let i = vec_to_i32(x);
    Ok(i)
}

fn day_to_i32<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let i = trans_week(&s);
    Ok(i)
}

fn index_time_to_i32<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let x = expand_time_collect(&s);
    let i = vec_to_i32(x);
    Ok(i)
}

fn str_to_vec_string<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let i = split_string(&s);
    Ok(i)
}

fn str_trim<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

pub fn parse_timetable_page(page: &str) -> Result<Vec<Course>> {
    let json_page: Value = serde_json::from_str(page)?;
    json_page["kbList"].as_array().map_or(Ok(vec![]), |course_list| {
        let result: std::result::Result<Vec<_>, serde_json::Error> = course_list
            .iter()
            .map(|course| serde_json::from_value::<Course>(course.clone()))
            .collect();
        Ok(result?)
    })
}
