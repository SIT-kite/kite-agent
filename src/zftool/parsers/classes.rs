use crate::zftool::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Major {
    #[serde(skip_deserializing)]
    entrance_year: i32,
    #[serde(skip_serializing, rename(deserialize = "njdm"))]
    /// 入学年份
    _entrance_year: String,
    #[serde(rename(deserialize = "zyh"))]
    /// 专业代码
    id: String,
    #[serde(rename(deserialize = "zymc"))]
    /// 专业名称
    name: String,
    #[serde(rename(deserialize = "zyh_id"))]
    /// 专业内部标识
    inner_id: String,
    #[serde(rename(deserialize = "zyfx_id"))]
    /// 专业方向内部表示
    direction_id: String,
    #[serde(rename(deserialize = "zyfxmc"))]
    /// 专业方向
    direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    #[serde(skip_deserializing)]
    grade: i32,
    #[serde(rename(deserialize = "njmc"))]
    /// 年级
    _grade: String,
    #[serde(rename(deserialize = "jgmc"))]
    /// 学院
    college: String,
    #[serde(rename(deserialize = "zymc"))]
    /// 专业名称
    major_name: String,
    #[serde(rename(deserialize = "zyh_id"))]
    /// 专业代码
    major_id: String,
    #[serde(rename(deserialize = "bh"))]
    /// 班级
    class_id: String,
}

pub fn parse_major_list_page(page: &str) -> Result<Vec<Major>> {
    let json_page: Value = serde_json::from_str(page)?;

    if let Some(major_list) = json_page.as_array() {
        let result = major_list
            .iter()
            .map(|v| {
                let mut x = serde_json::from_value::<Major>(v.clone()).unwrap();
                x.entrance_year = x._entrance_year.parse().unwrap();
                x
            })
            .collect();
        return Ok(result);
    }
    Ok(vec![])
}

pub fn parse_class_list_page(page: &str) -> Result<Vec<Class>> {
    let json_page: Value = serde_json::from_str(page)?;

    if let Some(major_list) = json_page.as_array() {
        let result = major_list
            .iter()
            .map(|v| {
                let mut x = serde_json::from_value::<Class>(v.clone()).unwrap();
                x.grade = x._grade.parse().unwrap();
                x
            })
            .collect();
        return Ok(result);
    }
    Ok(vec![])
}

#[test]
fn test_parse_major_list_page() {
    let page = r#"
[{
	"jgpxzd": "1",
	"listnav": "false",
	"localeKey": "zh_CN",
	"njdm": "2018",
	"njdm_id": "2018",
	"njmc": "2018",
	"pageable": true,
	"queryModel": {
		"currentPage": 1,
		"currentResult": 0,
		"entityOrField": false,
		"limit": 15,
		"offset": 0,
		"pageNo": 0,
		"pageSize": 15,
		"showCount": 10,
		"sorts": [],
		"totalCount": 0,
		"totalPage": 0,
		"totalResult": 0
	},
	"rangeable": true,
	"totalResult": "0",
	"userModel": {
		"monitor": false,
		"roleCount": 0,
		"roleKeys": "",
		"roleValues": "",
		"status": 0,
		"usable": false
	},
	"zyfx_id": "2018Y240101",
	"zyfxdm": "2018Y240101",
	"zyfxmc": "本科预科班",
	"zyh": "Y2401",
	"zyh_id": "Y2401",
	"zymc": "本科预科班"
}, {
	"jgpxzd": "1",
	"listnav": "false",
	"localeKey": "zh_CN",
	"njdm": "2018",
	"njdm_id": "2018",
	"njmc": "2018",
	"pageable": true,
	"queryModel": {
		"currentPage": 1,
		"currentResult": 0,
		"entityOrField": false,
		"limit": 15,
		"offset": 0,
		"pageNo": 0,
		"pageSize": 15,
		"showCount": 10,
		"sorts": [],
		"totalCount": 0,
		"totalPage": 0,
		"totalResult": 0
	},
	"rangeable": true,
	"totalResult": "0",
	"userModel": {
		"monitor": false,
		"roleCount": 0,
		"roleKeys": "",
		"roleValues": "",
		"status": 0,
		"usable": false
	},
	"zyfx_id": "2018B210000",
	"zyfxdm": "2018B210000",
	"zyfxmc": "人文学院大类(公共管理类、社会学类)",
	"zyh": "B2100",
	"zyh_id": "B2100",
	"zymc": "人文学院大类"
}]"#;

    let parsed_major_list = parse_major_list_page(page);
    println!("{:#?}", parsed_major_list);
}

#[test]
fn test_parse_class_list_page() {
    let page = r#"
[
    {
        "bh":"08108131",
        "bh_id":"08108131",
        "bj":"化妆品工艺1班",
        "jg_id":"08",
        "jgmc":"香料香精化妆品学部（香料香精技术与工程学院）",
        "jgpxzd":"1",
        "listnav":"false",
        "localeKey":"zh_CN",
        "njdm_id":"2008",
        "njmc":"2008",
        "pageable":true,
        "queryModel":{
            "currentPage":1,
            "currentResult":0,
            "entityOrField":false,
            "limit":15,
            "offset":0,
            "pageNo":0,
            "pageSize":15,
            "showCount":10,
            "sorts":[

            ],
            "totalCount":0,
            "totalPage":0,
            "totalResult":0
        },
        "rangeable":true,
        "totalResult":"0",
        "userModel":{
            "monitor":false,
            "roleCount":0,
            "roleKeys":"",
            "roleValues":"",
            "status":0,
            "usable":false
        },
        "xqh_id":"02",
        "zyh":"B0801",
        "zyh_id":"B0801",
        "zymc":"轻化工程"
    },
    {
        "bh":"99B06030101",
        "bh_id":"99B06030101",
        "bj":"毕业重修（环境）",
        "jg_id":"07",
        "jgmc":"化学与环境工程学院",
        "jgpxzd":"1",
        "listnav":"false",
        "localeKey":"zh_CN",
        "njdm_id":"1999",
        "njmc":"1999",
        "pageable":true,
        "queryModel":{
            "currentPage":1,
            "currentResult":0,
            "entityOrField":false,
            "limit":15,
            "offset":0,
            "pageNo":0,
            "pageSize":15,
            "showCount":10,
            "sorts":[

            ],
            "totalCount":0,
            "totalPage":0,
            "totalResult":0
        },
        "rangeable":true,
        "totalResult":"0",
        "userModel":{
            "monitor":false,
            "roleCount":0,
            "roleKeys":"",
            "roleValues":"",
            "status":0,
            "usable":false
        },
        "xqh_id":"01",
        "zyh":"B0701",
        "zyh_id":"B0701",
        "zymc":"化学工程与工艺"
    }
]"#;

    let parsed_class_list = parse_class_list_page(page);
    println!("{:#?}", parsed_class_list);
}
