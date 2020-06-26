mod error;
mod network;
mod user_agent;

#[macro_use]
extern crate lazy_static;

use crate::network::test_netwrok_connectivity;

// pub use crate::error::Result;
//
// use scraper::{Html, Selector};
// use std::fs::File;
// use std::io::Read;
//
// fn recent_activity() {
//     let html_page = std::fs::read_to_string("html\\第二课堂最近活动.html").unwrap();
//     let document = Html::parse_document(html_page.as_ref());
//     let selector = Selector::parse(".ul_7 li>a").unwrap();
//
//     for element in document.select(&selector) {
//         println!("活动标题 {}, 链接 {}", element.inner_html(), element.value().attr("href").unwrap());
//     }
// }
//
//
// fn my_score() {
//     let html_page = std::fs::read_to_string("html\\第二课堂得分页面.html").unwrap();
//     let document = Html::parse_document(html_page.as_ref());
//     let selector = Selector::parse("table[width=\"100%\"]>tbody>tr").unwrap();
//
//     // <td>如何防范校园贷</td>
//     // <td> <a title="-&quot;社团社区易班、学院活动&quot;"> 社团社区易班、...  </a> </td>
//     // <td>100887790 </td>
//     // <td>2018-9-26 18:57:12</td>
//     // <td> <span style="color:green">+0.1</span> </td>
//     // <td> <span style="color:green">+0.2</span> </td>
//     // <td style="color:red"></td>
//     for element in document.select(&selector) {
//         let mut all_columns = element.children();
//         println!("{}", all_columns.nth(0).unwrap().value().as_text().unwrap().text);
//     }
// }

#[actix_rt::main]
async fn async_main() {
    print!("{:?}", test_netwrok_connectivity().await)
}

fn main() {
    async_main()
}
