use super::Parse;
use scraper::{ElementRef, Html, Selector};

/// CoursePlan information from teaching plan.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoursePlan {
    /// Course unique identifier.
    pub course_code: String,
    /// Course name.
    pub course_name: String,
    /// Need exam or not.
    pub has_exam: bool,
    /// Credit.
    pub course_credit: f32,
    /// Theory lesson hours.
    pub theory_hour: u32,
    /// Practice lesson hours.
    pub practice_hour: u32,
    /// Department code of the Course.
    pub department_code: u32,
    /// Semester which the course on.
    pub semester: Vec<u32>,
}

impl CoursePlan {
    pub fn from(mut fields: Vec<String>) -> Self {
        let semester_option_fields: Vec<String> = fields.drain(8..18).collect();
        let semester: Vec<u32> = semester_option_fields
            .iter()
            .enumerate()
            .filter(|(i, n)| !n.is_empty())
            .map(|(i, v)| (i + 1) as u32)
            .collect();
        Self {
            course_code: fields[1].parse().unwrap_or_default(),
            course_name: fields[3].parse().unwrap_or_default(),
            has_exam: !fields[4].is_empty(),
            course_credit: fields[5].parse().unwrap_or_default(),
            theory_hour: fields[6].parse().unwrap_or_default(),
            practice_hour: fields[7].parse().unwrap_or_default(),
            department_code: fields[8].parse().unwrap_or_default(),
            semester,
        }
    }
}

impl Parse for Vec<CoursePlan> {
    fn from_html(html_page: &str) -> Self {
        let document = Html::parse_document(html_page);
        let table_selector =
            "body > div > table > tbody > tr:nth-child(3) > td > table > tbody".to_string();
        let table = document
            .select(&Selector::parse(table_selector.as_str()).unwrap())
            .next()
            .unwrap();

        let mut table_rows: Vec<ElementRef> = table.select(&Selector::parse("tr").unwrap()).collect();
        let table_rows: Vec<ElementRef> = table_rows.drain(1..(table_rows.len() - 2)).collect();

        let mut table_datas: Vec<Vec<String>> = table_rows
            .into_iter()
            .map(|e| {
                e.select(&Selector::parse("td").unwrap())
                    .map(|e| e.inner_html().trim().to_string())
                    .collect()
            })
            .collect();
        // deal with the major ID
        table_datas.iter_mut().for_each(|v| {
            let department_element = v[18].clone();
            v[18] = Html::parse_fragment(department_element.as_str())
                .select(&Selector::parse("span").unwrap())
                .next()
                .unwrap()
                .inner_html();
        });
        let results: Vec<CoursePlan> = table_datas.iter().map(|v| CoursePlan::from(v.clone())).collect();
        results
    }
}

#[cfg(test)]
mod test {
    use super::{CoursePlan, Parse};

    #[test]
    fn test_course_plan_parse() {
        let html_page = std::fs::read_to_string("html/教学计划查询页面.html").unwrap();
        let course_plans: Vec<CoursePlan> = Parse::from_html(html_page.as_str());

        let target = CoursePlan {
            course_code: "B123001".to_string(),
            course_name: "体育1".to_string(),
            has_exam: true,
            course_credit: 1.0,
            theory_hour: 32,
            practice_hour: 0,
            department_code: 23,
            semester: vec![1u32],
        };
        assert_eq!(course_plans[1], target)
    }
}
