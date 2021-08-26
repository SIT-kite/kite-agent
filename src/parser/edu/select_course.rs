use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCourse {
    #[serde(rename(deserialize = "kzmc"))]
    /// 课程名称
    course_name: String,
    #[serde(rename(deserialize = "kcmc"))]
    /// 实际课程名称
    sub_course_name: String,
    #[serde(rename(deserialize = "kch"))]
    /// 课程代码
    course_id: String,
    #[serde(rename(deserialize = "kklxdm"))]
    /// 开课学院
    college: String,
    #[serde(rename(deserialize = "yxzrs"))]
    /// 课程人数
    total_size: String,
    #[serde(rename(deserialize = "jxb_id"))]
    /// 课程序号(内部表示)
    inner_dyn_class_id: String,
    #[serde(rename(deserialize = "jxbmc"))]
    /// 课程序号
    dyn_class_id: String,
}

pub fn parse_available_course_page(page: &str) -> Result<Vec<SelectCourse>> {
    let json_page: Value = serde_json::from_str(page)?;

    if let Some(major_list) = json_page.as_array() {
        let result = major_list
            .iter()
            .map(|v| serde_json::from_value::<SelectCourse>(v.clone()).unwrap())
            .collect();
        return Ok(result);
    }
    Ok(vec![])
}
