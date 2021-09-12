use crate::error::Result;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref SCORE_DETAIL_PAGR: Selector =
        Selector::parse("div.table-responsive > #subtab > tbody > tr").unwrap();
    static ref SCORE_FORM: Selector = Selector::parse("td:nth-child(1)").unwrap();
    static ref SCORE_PERCENTAGE: Selector = Selector::parse("td:nth-child(2)").unwrap();
    static ref SCORE: Selector = Selector::parse("td:nth-child(3)").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreDetail {
    // 平时成绩
    score_type: String,
    // 期末成绩
    percentage: String,
    // 总评
    score: f32,
}

fn score_detail_map(item: ElementRef) -> Result<ScoreDetail> {
    let score_form: Option<String> = item
        .select(&SCORE_FORM)
        .next()
        .map(|x| replace_curly_brackets(x.inner_html().trim()));

    let score_percentage: Option<String> = item
        .select(&SCORE_PERCENTAGE)
        .next()
        .map(|x| replace_nbsp(x.inner_html().trim()));

    let score: Option<f32> = item
        .select(&SCORE)
        .next()
        .map(|x| replace_nbsp(x.inner_html().trim()).parse().unwrap_or_default());

    Ok(ScoreDetail {
        score_type: score_form.unwrap_or_default(),
        percentage: score_percentage.unwrap_or_default(),
        score: score.unwrap_or_default(),
    })
}

fn replace_nbsp(s: &str) -> String {
    s.replace("&nbsp;", "")
}

fn replace_curly_brackets(s: &str) -> String {
    s.replace("【 ", "").replace(" 】", "")
}

pub fn get_score_detail(html_page: &str) -> Result<Vec<ScoreDetail>> {
    let document = Html::parse_document(html_page);

    let result = document
        .select(&SCORE_DETAIL_PAGR)
        .map(score_detail_map)
        .collect::<Result<Vec<ScoreDetail>>>()?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::parser::edu::score_detail::{get_score_detail, parse_html, ScoreDetail};
    #[test]
    fn test_score_detail_parser() {
        let html_page = std::fs::read_to_string("html/成绩详情.html").unwrap();
        let origin: Vec<ScoreDetail> = get_score_detail(html_page.as_str()).unwrap();
        println!("{:?}", origin);
    }
}
