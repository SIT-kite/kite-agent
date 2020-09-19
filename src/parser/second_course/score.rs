use crate::error::Result;
use crate::parser::Parse;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug, Clone, PartialEq)]
pub struct SecondScore {
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

impl From<Vec<String>> for SecondScore {
    fn from(fields: Vec<String>) -> Self {
        Self {
            effect: fields[0].parse().unwrap_or_default(),
            total: fields[1].parse().unwrap_or_default(),
            integrity: fields[2].parse().unwrap_or_default(),
            theme_report: fields[3].parse().unwrap_or_default(),
            social_practice: fields[4].parse().unwrap_or_default(),
            creativity: fields[5].parse().unwrap_or_default(),
            safety_civilization: fields[6].parse().unwrap_or_default(),
            charity: fields[7].parse().unwrap_or_default(),
            campus_culture: fields[8].parse().unwrap_or_default(),
        }
    }
}

impl Parse for SecondScore {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);

        let display_score_vec = document
            .select(&Selector::parse("#content-box > div.user-info > div:nth-child(2) > font").unwrap())
            .map(|e| e.inner_html())
            .collect::<Vec<String>>();

        let hide_score_text = document
            .select(&Selector::parse("#span_score").unwrap())
            .next()
            .unwrap()
            .inner_html();

        let hide_score_re_vec = vec![
            Regex::new(r"=(\d+\.\d{0,2})\(主题报告\)")?,
            Regex::new(r"\+(\d+\.\d{0,2})\(社会实践\)")?,
            Regex::new(r"\+(\d+\.\d{0,2})\(创新创业创意\)")?,
            Regex::new(r"\+(\d+\.\d{0,2})\(校园安全文明\)")?,
            Regex::new(r"\+(\d+\.\d{0,2})\(公益志愿\)")?,
            Regex::new(r"\+(\d+\.\d{0,2})\(校园文化\)")?,
        ];

        let mut hide_score_vec = hide_score_re_vec
            .iter()
            .map(|r| {
                r.captures_iter(hide_score_text.as_str())
                    .map(|c| c.get(1).unwrap().as_str().to_string())
                    .nth(0)
                    .unwrap()
            })
            .collect::<Vec<String>>();

        // combine the two vec.
        let mut data = display_score_vec;
        data.append(&mut hide_score_vec);

        Ok(SecondScore::from(data))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_second_score_parser() {
        use super::{Parse, SecondScore};

        let html_page = std::fs::read_to_string("html/第二课堂得分页面.html").unwrap();
        let origin: SecondScore = Parse::from_html(html_page.as_str()).unwrap();
        let target = SecondScore {
            effect: 6.96,
            total: 10.62,
            integrity: 9.8,
            theme_report: 1.5,
            social_practice: 0.96,
            creativity: 1.5,
            safety_civilization: 1.0,
            charity: 0.0,
            campus_culture: 2.0,
        };
        assert_eq!(origin, target)
    }
}
