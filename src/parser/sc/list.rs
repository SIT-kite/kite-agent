use chrono::NaiveDateTime;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;

use crate::error::Result;
use crate::parser::Parse;

/// Activity link, used for list recent activities.
#[derive(Debug, Serialize)]
pub struct Activity {
    pub id: String,
    pub category: i32,
}

impl Parse for Vec<Activity> {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);
        let selector = Selector::parse(".ul_7 li > a").unwrap();
        let re = Regex::new(r"(\d){7}")?;

        let activities = document
            .select(&selector)
            .map(|each_line| {
                let link = each_line.value().attr("href").unwrap();

                Activity {
                    id: String::from(if let Some(id) = re.find(link) {
                        id.as_str()
                    } else {
                        ""
                    }),
                    category: 0,
                }
            })
            .collect();
        Ok(activities)
    }
}

#[derive(Debug)]
pub struct JoinedActivity {
    pub title: String,
    pub apply_id: String,
    pub apply_time: NaiveDateTime,
    pub score: f32,
}

impl Parse for Vec<JoinedActivity> {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);
        let selector = Selector::parse("table[width=\"100%\"] > tbody > tr").unwrap();

        let activities = document
            .select(&selector)
            .map(|each_line| {
                let cols: Vec<String> = each_line
                    .select(&Selector::parse("td").unwrap())
                    .map(|e| e.inner_html().trim().to_string())
                    .collect();
                let score_pattern = Regex::new(r"\+(\d+[.\d+]*)").unwrap();
                let score = score_pattern
                    .find(cols[4].as_ref())
                    .map(|x| x.as_str().parse::<f32>().unwrap());

                JoinedActivity {
                    title: cols[0].to_string(),
                    apply_id: cols[2].to_string(),
                    apply_time: NaiveDateTime::parse_from_str(cols[3].as_ref(), "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    score: score.unwrap_or_default(),
                }
            })
            .collect();
        Ok(activities)
    }
}
