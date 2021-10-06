use std::collections::HashMap;
use std::fmt::Debug;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::parser::Parse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HoldingPreview {
    /// 索书号
    #[serde(rename(deserialize = "callno"), default)]
    pub call_no: String,
    /// 所在馆代号
    #[serde(rename(deserialize = "curlib"), default)]
    library_code: String,
    /// 所在馆藏地点
    #[serde(rename(deserialize = "curlibName"), default)]
    library_name: String,
    /// 所在馆藏地点代号
    #[serde(rename(deserialize = "curlocal"), default)]
    location: String,
    /// 所在馆藏地点名
    #[serde(rename(deserialize = "curlocalName"), default)]
    location_name: String,
    /// 馆藏总数
    #[serde(rename(deserialize = "copycount"), default)]
    total: u32,
    /// 可借阅的数目
    #[serde(rename(deserialize = "loanableCount"), default)]
    loanable_count: u32,
    /// 书架号
    #[serde(rename(deserialize = "shelfno"), default)]
    shelf_no: String,
    /// 条码号
    #[serde(rename(deserialize = "barcode"), default)]
    barcode: String,
}

impl Default for HoldingPreview {
    fn default() -> Self {
        HoldingPreview {
            call_no: "".to_string(),
            library_code: "".to_string(),
            library_name: "".to_string(),
            location: "".to_string(),
            location_name: "".to_string(),
            total: 0,
            loanable_count: 0,
            shelf_no: "".to_string(),
            barcode: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(rename(deserialize = "doubanreview"), default)]
    douban_review: String,
    #[serde(rename(deserialize = "showBookPreview"), default)]
    show_book_preview: String,
    #[serde(rename(deserialize = "showBookSummary"), default)]
    show_book_summary: String,
    #[serde(rename(deserialize = "haveBookMetaResource"), default)]
    have_book_meta_resource: String,
    #[serde(rename(deserialize = "showBookCatalog"), default)]
    show_book_catalog: String,
    #[serde(rename(deserialize = "showBookAuthorIntroduction"), default)]
    show_book_author_introduction: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            douban_review: "".to_string(),
            show_book_preview: "".to_string(),
            show_book_summary: "".to_string(),
            have_book_meta_resource: "".to_string(),
            show_book_catalog: "".to_string(),
            show_book_author_introduction: "".to_string(),
        }
    }
}

/// 馆藏信息预览
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HoldingPreviews {
    /// 馆藏信息预览
    #[serde(rename(deserialize = "previews"), default)]
    pub holding_previews: HashMap<String, Vec<HoldingPreview>>,
    // /// 全局配置(不知道这是干啥的)
    // #[serde(rename(deserialize = "globalConfig"), default)]
    // pub global_config: GlobalConfig,
}

/// 图书信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    /// 图书号
    pub book_id: String,
    /// ISBN号
    pub isbn: String,
    /// 图书标题
    pub title: String,
    /// 图书作者
    pub author: String,
    /// 出版社
    pub publisher: String,
    /// 出版日期
    pub publish_date: String,
    /// 索书号
    pub call_no: String,
    // /// 馆藏信息
    // pub holding_preview: Vec<HoldingPreview>,
}

/// 检索结果
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchLibraryResult {
    /// 检索总结果数(所有页面的结果总数)
    pub result_count: u32,
    /// 检索用时
    pub use_time: f32,
    /// 当前页号
    pub current_page: u32,
    /// 总页数
    pub total_pages: u32,
    /// 当前页面图书列表
    pub book_list: Vec<Book>,
}

lazy_static! {
    static ref NUM_WITH_COMMA: Regex = Regex::new(r"(\d+,?)+").unwrap();
    static ref FLOAT_NUM: Regex = Regex::new(r"检索时间: (\d+(?:\.\d+)?)").unwrap();
    static ref CURRENT_PAGE: Selector =
        Selector::parse("div.meneame:nth-child(4) > b:nth-child(4)").unwrap();
    static ref TOTAL_PAGES: Selector =
        Selector::parse("div.meneame:nth-child(4) > span:nth-child(1)").unwrap();
    static ref RESULT_NUM_AND_TIME: Selector =
        Selector::parse("#search_meta > div:nth-child(1)").unwrap();
    static ref BOOK_LIST: Selector = Selector::parse(".resultTable > tbody:nth-child(1) > tr").unwrap();
    static ref BOOK_COVER_IMG: Selector = Selector::parse(".bookcover_img").unwrap();
    static ref BOOK_PUBLISH_DATE: Selector =
        Selector::parse("td:nth-child(4) > div:nth-child(1) > div:nth-child(3)").unwrap();
    static ref TITLE: Selector = Selector::parse(".title-link").unwrap();
    static ref AUTHOR: Selector = Selector::parse(".author-link").unwrap();
    static ref PUBLISHER: Selector = Selector::parse(".publisher-link").unwrap();
    static ref CALL_NO: Selector = Selector::parse(".callnosSpan").unwrap();
}

/// 页面元素解析为Book结构体
fn book_map_detail(item: ElementRef) -> Result<Book> {
    let bookcover_img = item.select(&BOOK_COVER_IMG).next().unwrap().value();
    let get_info_from_element = |selector: &Selector| {
        item.select(selector)
            .next()
            .unwrap()
            .inner_html()
            .trim()
            .to_string()
    };
    let publish_date = get_info_from_element(&BOOK_PUBLISH_DATE);
    let mut publish_date = publish_date.split("出版日期:");
    publish_date.next();
    let publish_date = publish_date.next().unwrap().trim().to_string();
    Ok(Book {
        book_id: bookcover_img.attr("bookrecno").unwrap().to_string(),
        isbn: bookcover_img.attr("isbn").unwrap().to_string(),
        title: get_info_from_element(&TITLE),
        author: get_info_from_element(&AUTHOR),
        publisher: get_info_from_element(&PUBLISHER),
        publish_date,
        call_no: get_info_from_element(&CALL_NO),
        // holding_preview: vec![],
    })
}

/// 将逗号分割的数字转换为u32整数
fn parse_number_with_comma(src: &str) -> Result<u32> {
    let mut result = "".to_string();
    for s in src.split(',') {
        result.push_str(s.trim());
    }
    Ok(result.parse::<u32>().unwrap())
}

#[test]
fn test_parse_number_with_comma() {
    assert_eq!(parse_number_with_comma("55,720").unwrap(), 55720);
}

/// 提取符合正则式的第一个字符串
fn regex_find<'a>(regex: &'a Regex, src: &'a str, index: usize) -> &'a str {
    regex
        .captures_iter(src)
        .next()
        .unwrap()
        .get(index)
        .unwrap()
        .as_str()
}

/// 从搜索页解析总页数
fn get_total_pages(document: &Html) -> Result<u32> {
    let content = document
        .select(&TOTAL_PAGES)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();
    let content = regex_find(&NUM_WITH_COMMA, content, 0);

    let content = parse_number_with_comma(content).unwrap();

    Ok(content)
}

/// 从搜索页解析检索结果数与检索时间
fn get_result_num_and_time(document: &Html) -> Result<(u32, f32)> {
    let mut content = document.select(&RESULT_NUM_AND_TIME);
    let mut content = content.next().unwrap().text();
    content.next();
    content.next();
    let content = content.next().unwrap();
    let result_count = regex_find(&NUM_WITH_COMMA, content, 0);
    let result_count = parse_number_with_comma(result_count).unwrap();
    let use_time = regex_find(&FLOAT_NUM, content, 1);
    let use_time = use_time.parse::<f32>().unwrap();
    Ok((result_count, use_time))
}

impl Parse for SearchLibraryResult {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);
        let book_list = document
            .select(&BOOK_LIST)
            .map(|x| book_map_detail(x).unwrap())
            .collect::<Vec<Book>>();
        let current_page = document
            .select(&CURRENT_PAGE)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let (result_count, use_time) = get_result_num_and_time(&document)?;
        Ok(SearchLibraryResult {
            result_count,
            book_list,
            current_page,
            total_pages: get_total_pages(&document).unwrap(),
            use_time,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{Parse, SearchLibraryResult};
    use crate::service::{SearchLibraryRequest, SortOrder, SortWay};
    use anyhow::Result;
    use reqwest;
    use tokio;

    #[tokio::test]
    async fn test() -> Result<()> {
        let url = SearchLibraryRequest::new()
            .keyword("Java")
            .sort_way(SortWay::PublishDate)
            .sort_order(SortOrder::Desc)
            .build_url();
        let response = reqwest::get(url).await?.text().await?;

        let s: SearchLibraryResult = Parse::from_html(&response.as_str()).unwrap();
        println!("{:#?}", s);
        Ok(())
    }
}
