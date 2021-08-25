use crate::zftool::parsers::ParserError;
use crate::zftool::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 学号
    student_no: String,
    /// 姓名
    name: String,
    /// 英文姓名
    name_eng: String,
    /// 性别
    sex: String,
    /// 证件类型
    credential_type: String,
    /// 证件号码
    credential_id: String,
    /// 出生日期
    birth_date: String,
    /// 民族
    ethnicity: String,
    /// 籍贯
    hometown: String,
    /// 入学日期
    enrollment_date: String,
    /// 学生类型
    types: String,
}

static ELEMENTS: [(&str, &str); 11] = [
    ("student_no", "#col_xh > p:nth-child(1)"),
    ("name", "#col_xm > p:nth-child(1)"),
    ("name_eng", "#col_ywxm > p:nth-child(1)"),
    ("sex", "#col_xbm > p:nth-child(1)"),
    ("credential_type", "#col_zjlxm > p:nth-child(1)"),
    ("credential_id", "#col_zjhm > p:nth-child(1)"),
    ("birth_date", "#col_csrq > p:nth-child(1)"),
    ("ethnicity", "#col_mzm > p:nth-child(1)"),
    ("hometown", "#col_jg > p:nth-child(1)"),
    ("enrollment_date", "#col_rxrq > p:nth-child(1)"),
    ("type", "#col_xslxdm > p:nth-child(1)"),
];

pub fn parse_profile_page(text: &str) -> Result<Profile> {
    use scraper::{Html, Selector};

    let pages = Html::parse_document(text);
    let mut values = Vec::new();

    for (_, selector) in ELEMENTS {
        let selectors = Selector::parse(selector).unwrap();
        let value = pages
            .select(&selectors)
            .next()
            .map(|x| x.inner_html().trim().to_string())
            .ok_or(ParserError::MissingField)?;
        values.push(value);
    }
    // It can be true that element.len() == ELEMENTS.len().
    let profile = Profile {
        student_no: Clone::clone(&values[0]),
        name: Clone::clone(&values[1]),
        name_eng: Clone::clone(&values[2]),
        sex: Clone::clone(&values[3]),
        credential_type: Clone::clone(&values[4]),
        credential_id: Clone::clone(&values[5]),
        birth_date: Clone::clone(&values[6]),
        ethnicity: Clone::clone(&values[7]),
        hometown: Clone::clone(&values[8]),
        enrollment_date: Clone::clone(&values[9]),
        types: Clone::clone(&values[10]),
    };
    Ok(profile)
}
