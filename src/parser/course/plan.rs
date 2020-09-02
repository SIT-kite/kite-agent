use super::Parse;
use crate::error::Result;
use scraper::{ElementRef, Html, Selector};

/// CoursePlan information from teaching plan.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlannedCourse {
    /// Course unique identifier.
    pub code: String,
    /// Course name.
    pub name: String,
    /// Need exam or not.
    pub has_exam: bool,
    /// Credit.
    pub credit: f32,
    /// Theory lesson hours.
    pub theory_hour: u32,
    /// Practice lesson hours.
    pub practice_hour: u32,
    /// Code of college or department the course affiliated.
    pub department_code: String,
    /// Semester which the course on.
    /// From 1 to 10 means 大一上, 大一下...
    pub semester: Option<u8>,
}

impl From<ElementRef<'_>> for PlannedCourse {
    fn from<'a>(line: ElementRef<'a>) -> Self {
        let cols_selector = Selector::parse("td").unwrap();
        // Convert each column to string and collect to Vec<String>.
        let cols: Vec<String> = line
            .select(&cols_selector)
            .map(|x| x.inner_html().trim().to_string())
            .collect();
        // Count the relevant columns and find the semester of the course.
        let mut semester: Option<u8> = None;
        for i in 8..18 {
            if !cols[i].is_empty() {
                semester = Some((i - 7) as u8);
                break;
            }
        }
        // The department code may looks like '<span title="体育教育部">23</span>'
        // Get the code '23' as a string
        let re = regex::Regex::new(r"(\d{2})").unwrap();
        let department = re.find(&cols[18]).map(|e| e.as_str().to_string());
        // Construct result
        PlannedCourse {
            code: cols[1].to_string(),
            name: cols[3].to_string(),
            has_exam: !cols[4].is_empty(),
            credit: cols[5].parse().unwrap_or(0.0),
            theory_hour: cols[6].parse().unwrap_or(0),
            practice_hour: cols[7].parse().unwrap_or(0),
            department_code: department.unwrap_or_default(),
            semester,
        }
    }
}
impl Parse for Vec<PlannedCourse> {
    fn from_html(html_page: &str) -> Result<Self> {
        let document = Html::parse_document(html_page);
        let row_selector = Selector::parse("tr[onclick='doClick(this);']").unwrap();
        // Select data lines.
        let results: Vec<PlannedCourse> = document
            .select(&row_selector)
            .map(|e| PlannedCourse::from(e))
            .collect();
        Ok(results)
    }
}

#[cfg(test)]
mod test {
    use super::{Parse, PlannedCourse};

    #[test]
    fn test_course_plan_parser() {
        let html_page = std::fs::read_to_string("html/教学计划查询页面.html").unwrap();
        let course_plans: Vec<PlannedCourse> = Parse::from_html(html_page.as_str());

        let target = PlannedCourse {
            code: "B123001".to_string(),
            name: "体育1".to_string(),
            has_exam: true,
            credit: 1.0,
            theory_hour: 32,
            practice_hour: 0,
            department_code: String::from("23"),
            semester: Some(1),
        };
        assert_eq!(course_plans[1], target)
    }
}
