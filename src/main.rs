#[macro_use]
extern crate lazy_static;

use std::any::Any;
use std::ops::Deref;

use scraper::node::Element;
use scraper::{Html, Node, Selector};

use crate::model::CourseScore;
use crate::network::test_network_connectivity;

mod error;
mod model;
mod network;
mod user_agent;

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

// <td>如何防范校园贷</td>
// <td> <a title="-&quot;社团社区易班、学院活动&quot;"> 社团社区易班、...  </a> </td>
// <td>100887790 </td>
// <td>2018-9-26 18:57:12</td>
// <td> <span style="color:green">+0.1</span> </td>
// <td> <span style="color:green">+0.2</span> </td>
// <td style="color:red"></td>
//     for element in document.select(&selector) {
//         let mut all_columns = element.children();
//         println!("{}", all_columns.nth(0).unwrap().value().as_text().unwrap().text);
//     }
// }

// // body > table > tbody > tr:nth-child(4) > td > table > tbody > tr:nth-child(5) > td:nth-child(1)
// fn test_parse_course() {
//     let html_page = std::fs::read_to_string("html\\成绩查询页面2.html").unwrap();
//
//     let document = Html::parse_document(html_page.as_ref());
//
//     let table_selector: String =
//         "body > table > tbody > tr:nth-child(4) > td > table > tbody ".to_string();
//     let table = document
//         .select(&Selector::parse(table_selector.as_ref()).unwrap())
//         .next()
//         .unwrap();
//     let table_rows = table.select(&Selector::parse("tr").unwrap()).collect::<Vec<_>>();
//     let table_datas = &table_rows
//         .into_iter()
//         .map(|t| {
//             t.select(&Selector::parse("td").unwrap())
//                 .map(|e| e.inner_html())
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>()[1..];
//     let mut course_score_vec: Vec<CourseScore> = Vec::with_capacity(0);
//     table_datas.into_iter().for_each( |data| {
//         let temp_course_score = CourseScore {
//             course_code: data[0].clone(),
//             course_name: data[1].clone(),
//             course_credit: data[2].clone(),
//             usual_grade: data[3].clone(),
//             mid_grade: data[4].clone(),
//             final_grade: data[5].clone(),
//             final_review: data[6].clone(),
//             second_final_grade: data[7].clone(),
//             second_final_review: data[8].clone(),
//         };
//         course_score_vec.push(temp_course_score.clone())
//     });
//     // for i in 0..table_datas.len() {
//     //     let temp_course_score = CourseScore {
//     //         course_code: table_datas[i][0].clone(),
//     //         course_name: table_datas[i][1].clone(),
//     //         course_credit: table_datas[i][2].clone(),
//     //         usual_grade: table_datas[i][3].clone(),
//     //         mid_grade: table_datas[i][4].clone(),
//     //         final_grade: table_datas[i][5].clone(),
//     //         final_review: table_datas[i][6].clone(),
//     //         second_final_grade: table_datas[i][7].clone(),
//     //         second_final_review: table_datas[i][8].clone(),
//     //     };
//     //     course_score_vec.push(temp_course_score.clone())
//     // }
//     println!("{:#?}", course_score_vec);
//     // println!("{:?}", table_datas.len());
//     // table_rows.into_iter().for_each(|td| println!("{:?}",td.inner_html()));
//     // print!("{:?}",td_iter);
//     // let table_rows = parent_table.children().filter(|e| e.value().is_element()).map(|e| e.value()).collect::<Vec<&Node>>();
//     // println!("{:?}", table_rows);
//     // for element in table_rows{
//     //     println!("{:?}",element.as_element().unwrap().)
//     // }
//     // for element in document.select(&Selector::parse(selector.as_ref()).unwrap()).childrens() {
//     //     // let mut all_columns = element.children();
//     //     // println!("{}", all_columns.nth(2).unwrap().value().as_text().unwrap().text);
//     // }
//     // println!("{:#?}",document.select(&Selector::parse(selector.as_ref()).unwrap()).next().unwrap().inner_html());
//
//     // let course_score_fields: Vec<String> = course_selector_vec
//     //     .into_iter()
//     //     .map(move |i| {
//     //         document
//     //             .select(&Selector::parse(i.as_ref()).unwrap())
//     //             .next()
//     //             .unwrap()
//     //             .inner_html()
//     //     })
//     //     .collect();
//     //
//     // let course_score = CourseScore {
//     //     course_code: course_score_fields[0].clone(),
//     //     course_name: course_score_fields[1].clone(),
//     //     course_credit: course_score_fields[2].clone(),
//     //     usual_grade: course_score_fields[3].clone(),
//     //     mid_grade: course_score_fields[4].clone(),
//     //     final_grade: course_score_fields[5].clone(),
//     //     final_review: course_score_fields[6].clone(),
//     //     second_final_grade: course_score_fields[7].clone(),
//     //     second_final_review: course_score_fields[8].clone(),
//     // };
//     // println!("{:?}", course_score);
// }

#[actix_rt::main]
async fn async_main() {
    print!("{:?}", test_network_connectivity().await)
}

fn main() {
    async_main()
}
