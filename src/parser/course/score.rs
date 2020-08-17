use super::Parse;
use scraper::{Html, Selector};

/// Course score function.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CourseScoreInner {
    /// Score got for daily performance
    pub regular_grade: f32,
    /// Midterm grade
    pub midterm_grade: f32,
    /// Final exam grade
    pub final_grade: f32,
    /// Total mark
    pub total_mark: f32,
    /// Make up exam score.
    pub make_up_grade: f32,
    /// Total mark after make-up exam
    pub make_up_total: f32,
}

#[derive(Debug, PartialEq)]
pub enum CourseScoreLine {
    /// Have commented the teacher
    Normal(CourseScoreInner),
    /// Comment (评教) is needed
    Uncomment,
}

#[derive(Debug, PartialEq)]
pub struct CourseScore {
    /// Unique ID of the course
    pub course_code: String,
    /// Course name
    pub course_name: String,
    /// credit
    pub course_credit: f32,
    /// score data.
    pub detail: CourseScoreLine,
}

impl From<&Vec<String>> for CourseScore {
    fn from(fields: &Vec<String>) -> Self {
        Self {
            course_code: fields[0].to_string(),
            course_name: fields[1].to_string(),
            course_credit: fields[2].parse().unwrap_or_default(),
            detail: if fields.contains(&"未评教".to_string()) {
                CourseScoreLine::Uncomment
            } else {
                CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: fields[3].parse().unwrap_or_default(),
                    midterm_grade: fields[4].parse().unwrap_or_default(),
                    final_grade: fields[5].parse().unwrap_or_default(),
                    total_mark: fields[6].parse().unwrap_or_default(),
                    make_up_grade: fields[7].parse().unwrap_or_default(),
                    make_up_total: fields[8].parse().unwrap_or_default(),
                })
            },
        } // return Self
    } // End of function: from
}

impl Parse for Vec<CourseScore> {
    fn from_html(html_page: &str) -> Self {
        // Read html page to parser.
        let document = Html::parse_document(html_page.as_ref());
        let table_selector: String =
            "body > table > tbody > tr:nth-child(4) > td > table > tbody ".to_string();

        // get table body element-refs.
        let table = document
            .select(&Selector::parse(table_selector.as_ref()).unwrap())
            .next()
            .unwrap();

        // get table rows element-refs from table body element-refs.
        let table_rows = table.select(&Selector::parse("tr").unwrap()).collect::<Vec<_>>();
        // put all table data(String) into a vector and get slice for remove the first row.
        let table_datas = &table_rows
            .into_iter()
            .map(|t| {
                t.select(&Selector::parse("td").unwrap())
                    .map(|e| e.inner_html().trim().to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()[1..];

        // Map lines into CourseScore struct.
        let result: Vec<CourseScore> = table_datas
            .into_iter()
            .map(|data| CourseScore::from(data))
            .collect();
        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::CourseScore;
    use super::{CourseScoreInner, CourseScoreLine};
    use crate::parser::Parse;

    #[test]
    fn test_course_score_parser() {
        let html_page = std::fs::read_to_string("html\\成绩查询页面2.html").unwrap();
        let origin_course_score_vec: Vec<CourseScore> = Parse::from_html(html_page.as_ref());
        let target_course_score_vec = vec![
            CourseScore {
                course_code: "B1310002".to_string(),
                course_name: "大学生就业与创业指导".to_string(),
                course_credit: 1.0,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 95.0,
                    midterm_grade: 0.0,
                    final_grade: 86.0,
                    total_mark: 89.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B3042236".to_string(),
                course_name: "软件测试技术".to_string(),
                course_credit: 2.5,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 96.0,
                    midterm_grade: 0.0,
                    final_grade: 82.0,
                    total_mark: 89.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B3042284".to_string(),
                course_name: "软件体系结构与设计模式".to_string(),
                course_credit: 2.5,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 95.0,
                    midterm_grade: 0.0,
                    final_grade: 79.0,
                    total_mark: 84.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B3042287".to_string(),
                course_name: "UI界面分析与设计".to_string(),
                course_credit: 2.5,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 90.0,
                    midterm_grade: 0.0,
                    final_grade: 93.0,
                    total_mark: 92.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B3042288".to_string(),
                course_name: "Web应用系统开发".to_string(),
                course_credit: 2.5,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 100.0,
                    midterm_grade: 0.0,
                    final_grade: 79.0,
                    total_mark: 85.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B4045109".to_string(),
                course_name: "Python基础".to_string(),
                course_credit: 2.0,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 92.0,
                    midterm_grade: 0.0,
                    final_grade: 87.0,
                    total_mark: 90.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B704206".to_string(),
                course_name: "计算机拆装与维护实训".to_string(),
                course_credit: 1.0,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 94.0,
                    midterm_grade: 0.0,
                    final_grade: 73.0,
                    total_mark: 86.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B7042659".to_string(),
                course_name: "软件工程实训".to_string(),
                course_credit: 3.0,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 0.0,
                    midterm_grade: 0.0,
                    final_grade: 85.0,
                    total_mark: 85.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
            CourseScore {
                course_code: "B7042664".to_string(),
                course_name: "Web应用系统开发课程设计".to_string(),
                course_credit: 2.0,
                detail: CourseScoreLine::Normal(CourseScoreInner {
                    regular_grade: 0.0,
                    midterm_grade: 0.0,
                    final_grade: 68.0,
                    total_mark: 68.0,
                    make_up_grade: 0.0,
                    make_up_total: 0.0,
                }),
            },
        ];
        assert_eq!(origin_course_score_vec, target_course_score_vec)
    }
}
