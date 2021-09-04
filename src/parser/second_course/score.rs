use std::collections::HashMap;

use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDateTime};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::error::Result;
use crate::parser::Parse;

const CLASSIFICATION: &[&str] = &[
    "主题报告",
    "社会实践",
    "创新创业创意",
    "校园安全文明",
    "公益志愿",
    "校园文化",
];

lazy_static! {
    static ref SCORE_SUMMARY_REGEX: Vec<Regex> = {
        CLASSIFICATION
            .iter()
            .map(|c| format!("(\\d+\\.\\d{{0,2}})\\({}\\)", c))
            .map(|pat| Regex::new(&pat).expect(&format!("Failed to generate pattern {}", pat)))
            .collect()
    };
    static ref ID_DETAIL: Selector = Selector::parse("td:nth-child(3)").unwrap();
    static ref SCORE_DETAIL: Selector = Selector::parse("td:nth-child(5) > span").unwrap();
    static ref SCORE_DETAIL_PAGE: Selector =
        Selector::parse("#div1 > div.table_style_4 > form > table:nth-child(4) > tbody > tr").unwrap();
    static ref TOTAL_SCORE: Selector =
        Selector::parse("#content-box > div.user-info > div:nth-child(2) > font").unwrap();
    static ref SPAN_SCORE: Selector = Selector::parse("#span_score").unwrap();
    static ref ACTIVITY_DETAIL: Selector = Selector::parse(
        "#content-box > div:nth-child(12) > div.table_style_4 > form > table > tbody > tr"
    )
    .unwrap();
    static ref ACTIVITY_ID_DETAIL: Selector = Selector::parse("td:nth-child(2)").unwrap();
    static ref TIME_DETAL: Selector = Selector::parse("td:nth-child(4)").unwrap();
    static ref STATUS_DETAIL: Selector = Selector::parse("td:nth-child(5)").unwrap();
    static ref ACTIVITY_ID: Regex = Regex::new(r"activityId=(\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScScoreSummary {
    /// Effective score.
    pub effect: f32,
    /// Total score.
    pub total: f32,
    /// Integrity score.
    pub integrity: f32,
    /// Subject report.(主题报告)
    pub theme_report: f32,
    /// Social practice.(社会实践)
    pub social_practice: f32,
    /// Innovation, entrepreneurship and creativity.(创新创业创意)
    pub creativity: f32,
    /// Campus safety and civilization.(校园安全文明)
    pub safety_civilization: f32,
    /// Charity and Volunteer.(公益志愿)
    pub charity: f32,
    /// Campus culture.(校园文化)
    pub campus_culture: f32,
}

impl From<Vec<String>> for ScScoreSummary {
    fn from(fields: Vec<String>) -> Self {
        let mapped_list: Vec<f32> = fields
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap_or_default())
            .collect();

        Self {
            effect: mapped_list[0],
            total: mapped_list[1],
            integrity: mapped_list[2],
            theme_report: mapped_list[3],
            social_practice: mapped_list[4],
            creativity: mapped_list[5],
            safety_civilization: mapped_list[6],
            charity: mapped_list[7],
            campus_culture: mapped_list[8],
        }
    }
}

impl Parse for ScScoreSummary {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);

        let display_score_vec = document
            .select(&TOTAL_SCORE)
            .map(|e| e.inner_html())
            .collect::<Vec<String>>();

        let hide_score_text = document.select(&SPAN_SCORE).next().unwrap().inner_html();

        let mut hide_score_vec = SCORE_SUMMARY_REGEX
            .iter()
            .map(|r| {
                r.captures_iter(hide_score_text.as_str())
                    .next()
                    .map(|c| c.get(1).unwrap().as_str().to_string())
                    .unwrap()
            })
            .collect::<Vec<String>>();

        // combine the two vec.
        let mut data = display_score_vec;
        data.append(&mut hide_score_vec);

        Ok(ScScoreSummary::from(data))
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScScoreItem {
    pub activity_id: i32,
    pub amount: f32,
}

fn score_map_detail(item: ElementRef) -> Result<ScScoreItem> {
    let id: Option<i32> = item
        .select(&ID_DETAIL)
        .next()
        .and_then(|x| Some(x.inner_html().trim().parse().unwrap_or_default()));

    let add_score: Option<f32> = item
        .select(&SCORE_DETAIL)
        .next()
        .and_then(|x| Some(x.inner_html().trim().parse().unwrap_or_default()));

    // TODO: Add error handler.
    Ok(ScScoreItem {
        activity_id: id.unwrap_or_default(),
        amount: add_score.unwrap_or_default(),
    })
}

fn filter_zero_score(x: &Result<ScScoreItem>) -> bool {
    if let Ok(e) = x {
        e.amount > 0.01
    } else {
        false
    }
}

pub fn get_score_detail(html_page: &str) -> Result<Vec<ScScoreItem>> {
    let document = Html::parse_document(html_page);
    let score_items = document
        .select(&SCORE_DETAIL_PAGE)
        .map(score_map_detail)
        .filter(filter_zero_score)
        .collect::<Result<Vec<ScScoreItem>>>()?;

    // Group and accumulate score by activity id.
    let map = score_items
        .into_iter()
        .fold(HashMap::<i32, f32>::new(), |mut map, x| {
            if let Some(old) = map.get_mut(&x.activity_id) {
                *old += x.amount;
            } else {
                map.insert(x.activity_id, x.amount);
            }
            map
        });

    let result = map
        .into_iter()
        .map(|(activity_id, amount)| ScScoreItem { activity_id, amount })
        .collect();
    Ok(result)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ScActivityItem {
    pub activity_id: i32,
    pub time: DateTime<Local>,
    pub status: String,
}

fn activity_map_detail(item: ElementRef) -> Result<ScActivityItem> {
    let activity_id: Option<i32> = item.select(&ACTIVITY_ID_DETAIL).next().and_then(|x| {
        ACTIVITY_ID.captures(x.inner_html().as_str()).map(|m| {
            m.get(1)
                .unwrap()
                .as_str()
                .trim()
                .to_string()
                .parse()
                .unwrap_or_default()
        })
    });

    let time: Option<DateTime<Local>> = item.select(&TIME_DETAL).next().and_then(|x| {
        let native_time =
            NaiveDateTime::parse_from_str(x.inner_html().trim(), "%Y-%m-%d %H:%M:%S").unwrap();
        let mut time = DateTime::from_utc(native_time, FixedOffset::east(8 * 3600));
        let eight_hour = Duration::hours(8);
        time = time - eight_hour;
        Some(time)
    });

    let status: Option<String> = item
        .select(&STATUS_DETAIL)
        .next()
        .and_then(|x| Some(String::from(x.inner_html().trim())));

    Ok(ScActivityItem {
        activity_id: activity_id.unwrap_or_default(),
        time: time.unwrap(),
        status: status.unwrap_or_default(),
    })
}

fn filter_delete_activity(x: &Result<ScActivityItem>) -> bool {
    if let Ok(e) = x {
        e.activity_id != 0
    } else {
        false
    }
}

pub fn get_activity_detail(html_page: &str) -> Result<Vec<ScActivityItem>> {
    let document = Html::parse_document(html_page);

    document
        .select(&ACTIVITY_DETAIL)
        .map(activity_map_detail)
        .filter(filter_delete_activity)
        .collect()
}
#[cfg(test)]
mod test {
    #[test]
    fn test_second_score_parser() {
        use super::{Parse, ScScoreSummary};

        let html_page = std::fs::read_to_string("html/第二课堂得分页面.html").unwrap();
        let origin: ScScoreSummary = Parse::from_html(html_page.as_str()).unwrap();
        let target = ScScoreSummary {
            effect: 5.85,
            total: 6.35,
            integrity: 1.7,
            theme_report: 1.35,
            social_practice: 1.1,
            creativity: 1.5,
            safety_civilization: 0.6,
            charity: 0.5,
            campus_culture: 0.8,
        };
        assert_eq!(origin, target)
    }

    #[test]
    fn test_score_detail() {
        use crate::parser::second_course::score::get_score_detail;
        let html_page = std::fs::read_to_string("html/第二课堂得分页面.html").unwrap();
        let detail = get_score_detail(&html_page);
        println!("{:?}", detail);
    }

    #[test]
    fn test_activity_detail() {
        use crate::parser::second_course::score::get_activity_detail;
        let html_page = std::fs::read_to_string("html/第二课堂得分活动页面.html").unwrap();
        let detail = get_activity_detail(&html_page);
        println!("{:?}", detail);
    }
}
