use crate::error::Result;
use crate::parser::edu::{str_to_f32, str_to_semester};
use crate::service::ActionError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Score {
    #[serde(rename(deserialize = "cj"), deserialize_with = "str_to_f32", default)]
    /// 成绩
    score: f32,
    #[serde(rename(deserialize = "kcmc"), default)]
    /// 课程
    course: String,
    #[serde(rename(deserialize = "kch"), default)]
    /// 课程代码
    course_id: String,
    #[serde(rename(deserialize = "jxb_id"), default)]
    /// 班级
    class_id: String,
    #[serde(rename(deserialize = "xnmmc"), default)]
    /// 学年
    school_year: String,
    #[serde(rename(deserialize = "xqm"), deserialize_with = "str_to_semester", default)]
    /// 学期
    semester: i32,
    #[serde(rename(deserialize = "xf"), deserialize_with = "str_to_f32", default)]
    /// 学分
    credit: f32,
}

pub fn parse_score_list_page(page: &str) -> Result<Vec<Score>> {
    let json_page: Value = serde_json::from_str(page)?;

    if let Some(course_list) = json_page["items"].as_array() {
        let result = course_list
            .iter()
            .map(|course| {
                serde_json::from_value::<Score>(course.clone())
                    .map_err(|_| ActionError::ParsingError.into())
            })
            .collect::<Result<Vec<Score>>>()?;
        Ok(result)
    } else {
        Ok(vec![])
    }
}

pub fn calculate_gpa(scores: Vec<Score>) -> f32 {
    let mut total_credits = 0.0;
    let mut t = 0.0;
    for s in scores {
        t += s.credit * s.score;
        total_credits += s.credit;
    }
    (t / total_credits / 10.0) - 5.0
}
