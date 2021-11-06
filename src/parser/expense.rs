use chrono::{DateTime, FixedOffset, Local, TimeZone};
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::parser::{Parse, ParserError};

/// Campus card consumption records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseRecord {
    /// Record date.
    pub ts: DateTime<Local>,
    /// Expense amount.
    pub amount: f32,
    /// Expense address.
    pub address: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PageInfo {
    /// current page number.
    pub current: u16,
    /// total pages number.
    pub total: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpensePage {
    pub records: Vec<ExpenseRecord>,
    /// Page information for current record.
    pub page: PageInfo,
}

impl Parse for ExpensePage {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);

        let pages_information: String = document
            .select(&Selector::parse("#listContent[align=right]").unwrap())
            .next()
            .ok_or_else(|| {
                ParserError::NoSuchElement("No \"#listContent[align=right]\" found.".to_string())
            })?
            .inner_html();

        let current_page_re = Regex::new(r"第(\d+)页")?;
        let total_pages_pages_re = Regex::new(r"共(\d+)页")?;

        let current_page = current_page_re
            .captures_iter(pages_information.as_str())
            .next()
            .map(|c| c.get(1).unwrap().as_str().to_string())
            .unwrap_or_default()
            .parse::<u16>()
            .unwrap_or_default();

        let total_pages = total_pages_pages_re
            .captures_iter(pages_information.as_str())
            .next()
            .map(|c| c.get(1).unwrap().as_str().to_string())
            .unwrap_or_default()
            .parse::<u16>()
            .unwrap_or_default();

        // Expense record.
        let frame = document
            .select(&Selector::parse("#table>tbody").unwrap())
            .next()
            .unwrap();

        // Records
        let items = frame
            .select(&Selector::parse("tr").unwrap())
            .map(|e| {
                e.select(&Selector::parse("td>div[align=center]").unwrap())
                    .map(|e| e.inner_html())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
            .drain(1..)
            .collect::<Vec<Vec<String>>>();

        // Vec<ExpenseRecord>.
        let res = items
            .iter()
            .map(|v| ExpenseRecord::from(v.clone()))
            .collect::<Vec<ExpenseRecord>>();

        Ok(Self {
            records: res,
            page: PageInfo {
                current: current_page,
                total: total_pages,
            },
        })
    }
}

impl From<Vec<String>> for ExpenseRecord {
    fn from(fields: Vec<String>) -> Self {
        let dt = format!("{} {}", fields[2], fields[3]);
        let dt = FixedOffset::east(8 * 3600)
            .datetime_from_str(&dt, "%Y-%m-%d %H:%M:%S")
            .unwrap();
        Self {
            ts: DateTime::<Local>::from(dt),
            amount: fields[4].parse().unwrap_or_default(),
            address: fields[5].parse().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::expense::ExpensePage;

    #[test]
    fn test_expense_record_parser() {
        use super::Parse;
        let html_page = std::fs::read_to_string("html/消费记录页面.html").unwrap();

        let origin: ExpensePage = Parse::from_html(html_page.as_str()).unwrap();

        println!("{:#?}", origin);
        // let target = ExpenseRecord {
        //     code: "学号位置".to_string(),
        //     name: "姓名位置".to_string(),
        //     date_time: chrono::NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
        //     time: chrono::NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
        //     amount: 9.9,
        //     address: "奉贤某食堂一层7#".to_string(),
        //     page_info: PageInfo {
        //         current_page: 1,
        //         total_pages: 2,
        //     },
        // };
        // assert_eq!(origin[0], target);
    }
}
