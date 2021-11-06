use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumVariantNames};

use crate::agent::SharedData;
use crate::parser::{HoldingPreviews, Parse, SearchLibraryResult};
use crate::service::{DoRequest, ResponsePayload, ResponseResult};

mod url {
    use const_format::concatcp;

    /// 图书馆url
    pub const HOME: &str = "http://210.35.66.106";

    /// 图书馆馆藏检索页面
    pub const OPAC: &str = concatcp!(HOME, "/opac");

    /// 搜索结果页
    pub const SEARCH: &str = concatcp!(OPAC, "/search");

    /// 图书信息页
    pub const BOOK: &str = concatcp!(OPAC, "/book");

    /// 借阅信息查询
    pub const HOLDING_PREVIEW: &str = concatcp!(BOOK, "/holdingPreviews");
}

/// 搜索方式
#[derive(EnumVariantNames, Debug, Display, Serialize, Deserialize)]
pub enum SearchWay {
    /// 按任意词查询
    #[strum(serialize = "")]
    Any,
    /// 标题名
    #[strum(serialize = "title")]
    Title,
    /// 正题名：一本书的主要名称
    #[strum(serialize = "title200a")]
    TitleProper,
    /// ISBN号
    #[strum(serialize = "isbn")]
    Isbn,
    /// 著者
    #[strum(serialize = "author")]
    Author,
    /// 主题词
    #[strum(serialize = "subject")]
    SubjectWord,
    /// 分类号
    #[strum(serialize = "class")]
    ClassNo,
    /// 控制号
    #[strum(serialize = "ctrlno")]
    CtrlNo,
    /// 订购号
    #[strum(serialize = "orderno")]
    OrderNo,
    /// 出版社
    #[strum(serialize = "publisher")]
    Publisher,
    /// 索书号
    #[strum(serialize = "callno")]
    CallNo,
}

/// 排序规则
#[derive(EnumVariantNames, Debug, Display, Serialize, Deserialize)]
pub enum SortWay {
    /// 匹配度
    #[strum(serialize = "score")]
    MatchScore,
    /// 出版日期
    #[strum(serialize = "pubdate_sort")]
    PublishDate,
    /// 主题词
    #[strum(serialize = "subject_sort")]
    Subject,
    /// 标题名
    #[strum(serialize = "title_sort")]
    Title,
    /// 作者
    #[strum(serialize = "author_sort")]
    Author,
    /// 索书号
    #[strum(serialize = "callno_sort")]
    CallNo,
    /// 标题名拼音
    #[strum(serialize = "pinyin_sort")]
    Pinyin,
    /// 借阅次数
    #[strum(serialize = "loannum_sort")]
    LoanCount,
    /// 续借次数
    #[strum(serialize = "renew_sort")]
    RenewCount,
    /// 题名权重
    #[strum(serialize = "title200Weight")]
    TitleWeight,
    /// 正题名权重
    #[strum(serialize = "title200aWeight")]
    TitleProperWeight,
    /// 卷册号
    #[strum(serialize = "title200h")]
    Volume,
}

#[derive(EnumVariantNames, Debug, Display, Serialize, Deserialize)]
pub enum SortOrder {
    /// 升序排序
    #[strum(serialize = "asc")]
    Asc,
    /// 降序排序
    #[strum(serialize = "desc")]
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchLibraryRequest {
    /// 搜索关键字
    keyword: String,
    /// 搜索结果数量
    rows: u16,
    /// 搜索分页号
    page: u32,
    /// 搜索方式
    search_way: SearchWay,
    /// 搜索结果的排序方式
    sort_way: SortWay,
    /// 搜索结果的升降序方式
    sort_order: SortOrder,
}

impl Default for SearchLibraryRequest {
    fn default() -> Self {
        SearchLibraryRequest {
            keyword: "".to_string(),
            rows: 10,
            page: 1,
            search_way: SearchWay::Any,
            sort_way: SortWay::MatchScore,
            sort_order: SortOrder::Desc,
        }
    }
}

impl SearchLibraryRequest {
    pub fn new() -> Self {
        SearchLibraryRequest::default()
    }

    pub fn keyword<T: ToString>(mut self, keyword: T) -> Self {
        self.keyword = keyword.to_string();
        self
    }

    pub fn rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn search_way(mut self, search_way: SearchWay) -> Self {
        self.search_way = search_way;
        self
    }

    pub fn sort_way(mut self, sort_way: SortWay) -> Self {
        self.sort_way = sort_way;
        self
    }
    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }
    pub fn build_url(&self) -> Url {
        Url::parse_with_params(
            url::SEARCH,
            [
                ("q", self.keyword.as_str()),
                ("searchType", "standard"),
                ("isFacet", "true"),
                ("view", "standard"),
                ("searchWay", self.search_way.to_string().as_str()),
                ("rows", self.rows.to_string().as_str()),
                ("sortWay", self.sort_way.to_string().as_str()),
                ("sortOrder", self.sort_order.to_string().as_str()),
                ("hasholding", "1"),
                ("searchWay0", "marc"),
                ("logical0", "AND"),
                ("page", self.page.to_string().as_str()),
            ],
        )
        .unwrap()
    }
}

#[async_trait::async_trait]
impl DoRequest for SearchLibraryRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let request = data.client.get(self.build_url()).build()?;
        let response = data.client.execute(request).await?;
        let html = response.text().await?;
        let books: SearchLibraryResult = Parse::from_html(&html)?;

        // let book_id_list = books.book_list
        //     .iter()
        //     .map(|x| x.book_id.clone())
        //     .collect::<Vec<String>>();

        // 获取馆藏表
        // let holding_previews = get_holding_previews(book_id_list, &data).await?;

        // 为book_list添加preview信息
        // books.book_list
        //     .iter_mut()
        //     .for_each(|x| {
        //         x.holding_preview = holding_previews
        //             .holding_previews
        //             .get(x.book_id.as_str())
        //             .unwrap()
        //             .clone();
        //     });
        Ok(ResponsePayload::SearchLibrary(books))
    }
}

/// 馆藏信息检索
#[derive(Debug, Serialize, Deserialize)]
pub struct BookHoldingRequest {
    book_id_list: Vec<String>,
}

impl Default for BookHoldingRequest {
    fn default() -> Self {
        BookHoldingRequest { book_id_list: vec![] }
    }
}

async fn get_holding_previews(book_id_list: Vec<String>, data: &SharedData) -> Result<HoldingPreviews> {
    let mut book_id_list_str = "".to_string();
    book_id_list.iter().for_each(|x| {
        book_id_list_str.push_str(x.as_str());
        book_id_list_str.push_str(",");
    });
    let url = Url::parse_with_params(
        url::HOLDING_PREVIEW,
        [
            ("bookrecnos", book_id_list_str),
            ("curLibcodes", "".to_string()),
            ("return_fmt", "json".to_string()),
        ],
    )
    .unwrap();

    let request = data.client.get(url).build()?;
    let response = data.client.execute(request).await?;
    let holding_preview = response.json::<HoldingPreviews>().await?;
    Ok(holding_preview)
}

/// 馆藏信息请求
#[async_trait::async_trait]
impl DoRequest for BookHoldingRequest {
    async fn process(self, data: SharedData) -> ResponseResult {
        let mut book_id_list_str = "".to_string();
        self.book_id_list.iter().for_each(|x| {
            book_id_list_str.push_str(x.as_str());
            book_id_list_str.push_str(",");
        });
        let url = Url::parse_with_params(
            url::HOLDING_PREVIEW,
            [
                ("bookrecnos", book_id_list_str),
                ("curLibcodes", "".to_string()),
                ("return_fmt", "json".to_string()),
            ],
        )
        .unwrap();

        let request = data.client.get(url).build()?;
        let response = data.client.execute(request).await?;
        let holding_previews = response.json::<HoldingPreviews>().await?;
        Ok(ResponsePayload::BookHoldingInfo(holding_previews))
    }
}
