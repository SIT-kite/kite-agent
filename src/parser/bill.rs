use crate::error::Result;
use crate::parser::Parse;
use scraper::{Html, Selector};
use serde::Serialize;

/// Electricity bill
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct ElectricityBill {
    /// Room id in the format which described in the doc.
    pub room: String,
    /// Remaining paid amount
    pub balance: f32,
    /// Remaining subsidy amount
    pub subsidy: f32,
    /// Total available amount
    pub total: f32,
    /// Available power
    pub power: f32,
}

impl Parse for ElectricityBill {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page.as_ref());
        let selector = Selector::parse("#table tr td div").unwrap();
        let err_selector = Selector::parse("#notFound span").unwrap();

        if document.select(&err_selector).count() != 0 {
            return Ok(Self::default());
        }
        let cols: Vec<String> = document
            .select(&selector)
            .map(|x| x.inner_html().to_string())
            .collect();

        Ok(Self {
            room: cols[0].clone(),
            balance: cols[1].parse()?,
            subsidy: cols[2].parse()?,
            total: cols[3].parse()?,
            power: cols[4].parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::ElectricityBill;
    use super::Parse;

    #[test]
    pub fn test_electricity_bill_parser() {
        let file = std::fs::read_to_string("html/电费查询页面.html").unwrap();
        let bill: ElectricityBill = Parse::from_html(file.as_ref()).unwrap();

        assert_eq!(
            bill,
            ElectricityBill {
                room: "000000".to_string(),
                balance: 0.0,
                subsidy: 0.0,
                total: 0.0,
                power: 0.0,
            }
        )
    }
}
