use crate::error::{CrawlerError, Result};
use crate::parsers::{ParserError, TryParse};
use chrono::NaiveDateTime;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

/// Activity link, used for list recent activities.
#[derive(Debug)]
pub struct ActivityDetail {
    /// Activity id
    pub id: String,
    /// Activity title
    pub title: String,
    /// Activity start date time
    pub start_time: Option<NaiveDateTime>,
    /// Sign date time
    pub sign_time: Option<NaiveDateTime>,
    /// Activity end date time
    pub end_time: Option<NaiveDateTime>,
    /// Place
    pub place: Option<String>,
    /// Duration
    pub duration: Option<String>,
    /// Activity manager
    pub manager: Option<String>,
    /// Manager contact (phone)
    pub contact: Option<String>,
    /// Activity organizer
    pub organizer: Option<String>,
    /// Acitvity undertaker
    pub undertaker: Option<String>,
    /// Description in text[]
    pub description: Vec<String>,
    // pub attachment
}

#[inline]
fn regex_find_one(re: Regex, text: &str) -> Result<String> {
    re.captures(text)
        .ok_or(ParserError::RegexErr(format!("表达式 {}", re.as_str())).into())
        .and_then(|cap| Ok(cap.get(1).unwrap().as_str().trim().to_string()))
}

impl TryParse for ActivityDetail {
    fn try_from_html(html_page: &str) -> Result<ActivityDetail> {
        let document = Html::parse_document(html_page);

        // It is not difficult to find that the entire page is in a div container which is of class "box-1"
        // The three elements in that div: title, banner(some details) and body(description)
        // So our goal is clear now.
        let frame: ElementRef = document
            .select(&Selector::parse(".box-1").unwrap())
            .nth(0)
            .ok_or(CrawlerError::from(ParserError::NoSuchElement(String::from(
                ".box-1",
            ))))?;

        // Title
        let title = frame
            .select(&Selector::parse("h1").unwrap())
            .nth(0)
            .unwrap()
            .inner_html();
        // Banner
        let banner = frame
            .select(&Selector::parse("div[style=\" color:#7a7a7a; text-align:center\"]").unwrap())
            .nth(0)
            .unwrap()
            .inner_html()
            .replace("&nbsp;", "")
            .replace("<br>", "\n");
        // Description
        let body = frame
            .select(&Selector::parse("div[style=\"padding:30px 50px; font-size:14px;\"]").unwrap())
            .nth(0)
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .join("")
            .replace("\u{a0}", "");

        let sign_end_time = Regex::new(r"刷卡时间段：(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+).*--至--.*(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+)").unwrap()
            .captures(banner.as_ref()).ok_or(ParserError::RegexErr(String::from("解析刷卡时间")))?;

        Ok(ActivityDetail {
            id: regex_find_one(Regex::new(r"活动编号：(\d{7})")?, &banner)?,
            title: title,
            start_time: NaiveDateTime::parse_from_str(
                &regex_find_one(
                    Regex::new(r"活动开始时间：(\d{4}-\d{1,2}-\d{1,2} \d+:\d+:\d+)")?,
                    &banner,
                )?,
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
            sign_time: NaiveDateTime::parse_from_str(
                sign_end_time.get(1).unwrap().as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
            end_time: NaiveDateTime::parse_from_str(
                sign_end_time.get(2).unwrap().as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .ok(),
            place: regex_find_one(Regex::new(r"活动地点：(.*)")?, &banner).ok(),
            duration: regex_find_one(Regex::new(r"活动时长：(.*)")?, &banner).ok(),
            manager: regex_find_one(Regex::new(r"负责人：(.*)")?, &banner).ok(),
            contact: regex_find_one(Regex::new(r"负责人电话：(.*)")?, &banner).ok(),
            organizer: regex_find_one(Regex::new(r"主办方：(.*)")?, &banner).ok(),
            undertaker: regex_find_one(Regex::new(r"承办方：(.*)")?, &banner).ok(),
            description: Regex::new("[\n\t ]*")
                .unwrap()
                .replace(body.as_ref(), "\n")
                .split("\n")
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect(),
        })
    }
}
