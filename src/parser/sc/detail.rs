use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Local, TimeZone};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::error::Result;
use crate::parser::{Parse, ParserError};
use crate::service::ActionError;

lazy_static! {
    static ref RE_SPACES: Regex = Regex::new(r"\s{2}\s+").unwrap();
    // <img alt="" src="/js/kindeditor-4.1.7/attached/image/20200528/20200528101316_172.png">
    static ref RE_IMAGES: Regex = Regex::new(r#"<img(?:.*)src="(.*)">"#).unwrap();
    static ref SELECTOR_FRAME: Selector = Selector::parse(".box-1").unwrap();
    static ref SELECTOR_TITLE: Selector = Selector::parse("h1").unwrap();
    static ref SELECTOR_BANNER: Selector =
        Selector::parse("div[style=\" color:#7a7a7a; text-align:center\"]").unwrap();
    static ref SELECTOR_DESCRIPTION: Selector =
        Selector::parse("div[style=\"padding:30px 50px; font-size:14px;\"]").unwrap();
}

/// Activity link, used for list recent activities.
#[derive(serde::Serialize, Debug)]
pub struct ActivityDetail {
    /// Activity id
    pub id: i32,
    /// Category id
    pub category: i32,
    /// Activity title
    pub title: String,
    /// Activity start date time
    pub start_time: DateTime<Local>,
    /// Sign date time
    pub sign_start_time: DateTime<Local>,
    /// Activity end date time
    pub sign_end_time: DateTime<Local>,
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
    /// Activity undertaker
    pub undertaker: Option<String>,
    /// Description in text[]
    pub description: String,
    /// Image attachment.
    pub images: Vec<ScImages>,
}

#[derive(serde::Serialize, Debug)]
pub struct ScImages {
    pub new_name: String,
    pub old_name: String,
    pub content: Vec<u8>,
}
fn clean_text(banner: &str) -> String {
    let banner = banner.replace("&nbsp;", " ");
    let banner = banner.replace("<br>", "");
    RE_SPACES.replace_all(&banner, "\n").to_string()
}

fn split_key_value(line: &str) -> (String, String) {
    let (key, value) = line.split_once("：").unwrap_or_default();

    (key.to_string(), value.to_string())
}

fn split_activity_properties(banner: &str) -> HashMap<String, String> {
    let clean_text = clean_text(banner);

    clean_text
        .lines()
        .map(split_key_value)
        .fold(HashMap::new(), |mut map, (k, v)| {
            map.insert(k, v);
            map
        })
}

fn parse_date_time(date_time: &str) -> DateTime<Local> {
    let tz = FixedOffset::east(8 * 3600);
    let dt = tz.datetime_from_str(date_time, "%Y-%m-%d %H:%M:%S").unwrap();

    DateTime::<Local>::from(dt)
}

fn parse_sign_time(value: &str) -> (DateTime<Local>, DateTime<Local>) {
    let (start_s, end_s) = value.split_once("  --至--  ").unwrap();

    (parse_date_time(start_s), parse_date_time(end_s))
}

fn parse_properties(banner: &str) -> ActivityDetail {
    let properties = split_activity_properties(banner);
    let to_o = |x: &String| if x.is_empty() { None } else { Some(x.to_string()) };

    let sign_time = parse_sign_time(&properties["刷卡时间段"]);
    ActivityDetail {
        id: properties["活动编号"].parse().unwrap_or_default(),
        category: 0,
        title: "".to_string(),
        start_time: parse_date_time(&properties["活动开始时间"]),
        sign_start_time: sign_time.0,
        sign_end_time: sign_time.1,
        place: to_o(&properties["活动地点"]),
        duration: to_o(&properties["活动时长"]),
        manager: to_o(&properties["负责人"]),
        contact: to_o(&properties["负责人电话"]),
        organizer: to_o(&properties["主办方"]),
        undertaker: to_o(&properties["承办方"]),
        description: "".to_string(),
        images: vec![],
    }
}

fn select_text(e: ElementRef, selector: &Selector) -> String {
    e.select(selector)
        .next()
        .map(|x| x.inner_html())
        .unwrap_or_default()
}

fn replace_images(html: &str) -> (String, Vec<ScImages>) {
    // Find all images and generate uuid for each of them.
    let images = RE_IMAGES
        .captures_iter(html)
        .map(|src| {
            let old_name = src[1].to_string();
            let (_, file_extension) = old_name.rsplit_once(".").unwrap_or_default();
            let new_name = format!("{}.{}", uuid::Uuid::new_v4().to_string(), file_extension);
            ScImages {
                new_name,
                old_name,
                content: vec![],
            }
        })
        .collect::<Vec<_>>();

    let mut html = html.to_string();
    // Replace old image path to new one
    for image in images.iter() {
        html = html.replace(&image.old_name, &image.new_name);
    }

    (html, images)
}

fn parse_description(frame: ElementRef) -> (String, Vec<ScImages>) {
    let description = select_text(frame, &SELECTOR_DESCRIPTION);
    let (description, images) = replace_images(&description);

    (description, images)
}

impl Parse for ActivityDetail {
    fn from_html(html_page: &str) -> Result<ActivityDetail> {
        let document = Html::parse_document(html_page);

        // It is not difficult to find that the entire page is in a div container which is of class "box-1"
        // The three elements in that div: title, banner(some details) and body(description)
        // So our goal is clear now.
        let frame: ElementRef = document
            .select(&SELECTOR_FRAME)
            .next()
            .ok_or_else(|| ParserError::NoSuchElement(String::from(".box-1")))?;

        let title = select_text(frame, &SELECTOR_TITLE);
        let banner = select_text(frame, &SELECTOR_BANNER);
        let (description, images) = parse_description(frame);

        let mut result = parse_properties(&banner);
        result.title = title;
        result.description = description;
        result.images = images;
        Ok(result)
    }
}

pub enum ScJoinResult {
    Ok,
    Err(String),
}

impl Parse for ScJoinResult {
    fn from_html(html_page: &str) -> Result<ScJoinResult> {
        let code = html_page.parse::<i32>().map_err(|_| ActionError::ParsingError)?;
        if code == 0 {
            return Ok(ScJoinResult::Ok);
        }
        let message = match code {
            1 => "您的个人信息不全，请补全您的信息！",
            2 => "您已申请过该活动，不能重复申请！",
            3 => "对不起，您今天的申请次数已达上限！",
            4 => "对不起，该活动的申请人数已达上限！",
            5 => "对不起，该活动已过期并停止申请！",
            6 => "您已申请过该时间段的活动，不能重复申请！",
            7 => "对不起，您不能申请该活动！",
            8 => "对不起，您不在该活动的范围内！",
            _ => "未知错误",
        };
        Ok(ScJoinResult::Err(message.to_string()))
    }
}

#[test]
fn test_activity_detail() {
    let html_page = std::fs::read_to_string("html/第二课堂详情页面2.html").unwrap();
    let detail = ActivityDetail::from_html(&html_page);
    println!("{:?}", detail);
}
