use crate::parsers::Parse;
use scraper::{Html, Selector};

/// Electricity bill
#[derive(Debug, PartialEq)]
pub struct ElectricityBill {
    pub room_id: String,
    pub deposit_balance: f32,
    pub subsidized_balance: f32,
    pub total_balance: f32,
    pub available_power: f32,
}

impl Parse for ElectricityBill {
    fn from_html(html_page: &str) -> Self {
        let document = Html::parse_document(html_page.as_ref());
        let selector = Selector::parse("#table tr td div").unwrap();
        let cols: Vec<String> = document
            .select(&selector)
            .map(|x| x.inner_html().to_string())
            .collect();

        Self {
            room_id: cols[0].clone(),
            deposit_balance: cols[1].parse().unwrap(),
            subsidized_balance: cols[2].parse().unwrap(),
            total_balance: cols[3].parse().unwrap(),
            available_power: cols[4].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ElectricityBill;
    use super::Parse;

    #[test]
    pub fn test_electricity_bill_parser() {
        let file = std::fs::read_to_string("html/电费查询页面.html").unwrap();
        let bill: ElectricityBill = Parse::from_html(file.as_ref());

        assert_eq!(
            bill,
            ElectricityBill {
                room_id: "000000".to_string(),
                deposit_balance: 0.0,
                subsidized_balance: 0.0,
                total_balance: 0.0,
                available_power: 0.0,
            }
        )
    }
}
