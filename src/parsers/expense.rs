use crate::parsers::Parse;
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use scraper::{Html, Selector};

/// Campus card consumption records
#[derive(Debug, Clone, PartialEq)]
pub struct ExpenseRecord {
    pub code: String,
    pub name: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub amount: f32,
    pub address: String,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageInfo {
    /// current page number.
    pub current_page: u32,
    /// total pages number.
    pub total_pages: u32,
}

impl Parse for Vec<ExpenseRecord> {
    fn from_html(html_page: &str) -> Self {
        let document = Html::parse_document(html_page);

        let pages_information: String = document
            .select(&Selector::parse("#listContent[align=right]").unwrap())
            .next()
            .unwrap()
            .inner_html();

        let current_page_re = Regex::new(r"第(\d+)页").unwrap();
        let total_pages_pages_re = Regex::new(r"共(\d+)页").unwrap();

        let current_page = current_page_re
            .captures_iter(pages_information.as_str())
            .map(|c| c.get(1).unwrap().as_str().to_string())
            .nth(0)
            .unwrap_or_default();

        let total_pages = total_pages_pages_re
            .captures_iter(pages_information.as_str())
            .map(|c| c.get(1).unwrap().as_str().to_string())
            .nth(0)
            .unwrap_or_default();

        // Expense record.
        let frame = document
            .select(&Selector::parse("#table>tbody").unwrap())
            .next()
            .unwrap();

        // Record
        let mut datas = frame
            .select(&Selector::parse("tr").unwrap())
            .map(|e| {
                e.select(&Selector::parse("td>div[align=center]").unwrap())
                    .map(|e| e.inner_html())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
            .drain(1..)
            .collect::<Vec<Vec<String>>>();

        // Add page info.
        datas.iter_mut().for_each(|v| {
            v.push(current_page.clone());
            v.push(total_pages.clone());
        });

        // Return Vec<ExpenseRecord>.
        let res = datas
            .iter()
            .map(|v| ExpenseRecord::from(v.clone()))
            .collect::<Vec<ExpenseRecord>>();

        res
    }
}

impl From<Vec<String>> for ExpenseRecord {
    fn from(fields: Vec<String>) -> Self {
        Self {
            code: fields[0].parse().unwrap_or_default(),
            name: fields[1].parse().unwrap_or_default(),
            date: NaiveDate::parse_from_str(fields[2].as_str(), "%Y-%m-%d")
                .unwrap_or(NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap()),
            time: NaiveTime::parse_from_str(fields[3].as_str(), "%H:%M:%S")
                .unwrap_or(NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()),
            amount: fields[4].parse().unwrap_or_default(),
            address: fields[5].parse().unwrap_or_default(),
            page_info: PageInfo {
                current_page: fields[6].parse().unwrap(),
                total_pages: fields[7].parse().unwrap(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_expense_record_parser() {
        use super::Parse;
        use super::{ExpenseRecord, PageInfo};
        let html_page = std::fs::read_to_string("html/消费记录页面.html").unwrap();

        let origin: Vec<ExpenseRecord> = Parse::from_html(html_page.as_str());

        let target = ExpenseRecord {
            code: "学号位置".to_string(),
            name: "姓名位置".to_string(),
            date: chrono::NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
            time: chrono::NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
            amount: 9.9,
            address: "奉贤某食堂一层7#".to_string(),
            page_info: PageInfo {
                current_page: 1,
                total_pages: 2,
            },
        };
        assert_eq!(origin[0], target);
    }
}
