use crate::parsers::Parse;
use scraper::{Html, Selector};

/// Course selection on current semester.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedCourse {
    /// Course name.
    pub name: String,
    /// Course credit
    pub credit: f32,
    /// Course code.
    pub code: String,
    /// Course class id(which includes time info).
    pub class_id: String,
    /// The teacher name of the course.
    pub teacher_name: String,
    /// The Q&A time info of the course.
    pub qa_time: String,
    /// note
    pub note: String,
}

impl From<Vec<String>> for SelectedCourse {
    fn from(fields: Vec<String>) -> Self {
        Self {
            name: fields[0].parse().unwrap_or_default(),
            credit: fields[1].parse().unwrap_or_default(),
            code: fields[2].parse().unwrap_or_default(),
            class_id: fields[3].parse().unwrap_or_default(),
            teacher_name: fields[4].parse().unwrap_or_default(),
            qa_time: fields[5].trim().to_string().parse().unwrap_or_default(),
            note: fields[6].parse().unwrap_or_default(),
        }
    }
}

impl Parse for Vec<SelectedCourse> {
    fn from_html(html_page: &str) -> Self {
        let document = Html::parse_document(html_page);

        let fragment = document
            .select(&Selector::parse("body > form > table:nth-child(6) > tbody").unwrap())
            .next()
            .unwrap();

        // SelectedCourse data
        let data = fragment
            .select(&Selector::parse("tr[bgcolor=white]").unwrap())
            .map(|e| {
                e.select(&Selector::parse("td").unwrap())
                    .map(|e| e.inner_html())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
            .drain(2..)
            .collect::<Vec<Vec<String>>>();

        // Vec<SelectedCourse>
        let res = data
            .iter()
            .map(|v| SelectedCourse::from(v.clone()))
            .collect::<Vec<SelectedCourse>>();

        res
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_selected_course_parser() {
        use super::Parse;
        use super::{Html, SelectedCourse, Selector};

        let html_page = std::fs::read_to_string("html/我的课表页面.html").unwrap();

        let origin: Vec<SelectedCourse> = Parse::from_html(html_page.as_str());

        let target = SelectedCourse {
            name: "大学物理实验2".to_string(),
            credit: 1.0,
            code: "B1221026".to_string(),
            class_id: "1900733".to_string(),
            teacher_name: "刘聚坤".to_string(),
            qa_time: "1-16周全周;星期四;11:05-11:50;奉贤校区第四学科楼_218;1-16周全周;星期一;11:05-11:50;奉贤校区第四学科楼_218;1-16周全周;星期二;08:20-11:50;奉贤校区第四学科楼_218;1-16周全周;星期四;12:00-13:00;奉贤校区第四学科楼_218;1-16周全周;星期一;12:00-13:00;奉贤校区第四学科楼_218;1-16周全周;星期五;12:00-13:00;奉贤校区第四学科楼_218;1-16周全周;星期二;13:00-16:30;奉贤校区第四学科楼_218;1-16周全周;星期二;12:00-13:00;奉贤校区第四学科楼_218".to_string(),
            note: "".to_string(),
        };

        assert_eq!(origin[0], target)
    }
}
