use scraper::{Html, Selector};
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct User {
    // 学号
    username: String,
    // 密码
    password: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CourseScore {
    // 课程代码
    pub course_code: String,
    // 课程名称
    pub course_name: String,
    // 学分
    pub course_credit: String,
    // 平时成绩
    pub usual_grade: String,
    // 期中成绩
    pub mid_grade: String,
    // 期末成绩
    pub final_grade: String,
    // 期末总评
    pub final_review: String,
    // 二考成绩
    pub second_final_grade: String,
    // 二考总评
    pub second_final_review: String,
}
impl CourseScore {
    fn get_from_file(html_page: String) -> Vec<CourseScore> {
        // get html source
        let document = Html::parse_document(html_page.as_ref());

        // define a selector string for table body.
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
                    .map(|e| e.inner_html())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()[1..];

        let mut course_score_vec: Vec<CourseScore> = Vec::with_capacity(0);

        // make the data into CourseScore struct.
        table_datas.into_iter().for_each(|data| {
            let temp_course_score = CourseScore {
                course_code: data[0].clone(),
                course_name: data[1].clone(),
                course_credit: data[2].clone(),
                usual_grade: data[3].clone(),
                mid_grade: data[4].clone(),
                final_grade: data[5].clone(),
                final_review: data[6].clone(),
                second_final_grade: data[7].clone(),
                second_final_review: data[8].clone(),
            };
            course_score_vec.push(temp_course_score.clone())
        });

        course_score_vec
    }
}

impl Display for CourseScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CourseScore:
                  course_code: {},
                  course_name: {},
                  course_credit:{},
                  usual_grade: {},
                  mid_grade: {},
                  final_grade: {},
                  final_review: {},
                  second_final_grade: {},
                  second_final_review: {}",
            self.course_code,
            self.course_name,
            self.course_credit,
            self.usual_grade,
            self.mid_grade,
            self.final_grade,
            self.final_review,
            self.second_final_grade,
            self.second_final_review
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::model::CourseScore;

    #[test]
    fn test_get_from_file() {
        let html_page = std::fs::read_to_string("html\\成绩查询页面2.html").unwrap();
        let origin_course_score_vec = CourseScore::get_from_file(html_page);
        let target_course_score_vec = vec![
            CourseScore {
                course_code: "B1310002".to_string(),
                course_name: "大学生就业与创业指导".to_string(),
                course_credit: "1".to_string(),
                usual_grade: "95".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "86\n\n".to_string(),
                final_review: "89".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B3042236".to_string(),
                course_name: "软件测试技术".to_string(),
                course_credit: "2.5".to_string(),
                usual_grade: "96".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "82\n\n".to_string(),
                final_review: "89".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B3042284".to_string(),
                course_name: "软件体系结构与设计模式".to_string(),
                course_credit: "2.5".to_string(),
                usual_grade: "95".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "79\n\n".to_string(),
                final_review: "84".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B3042287".to_string(),
                course_name: "UI界面分析与设计".to_string(),
                course_credit: "2.5".to_string(),
                usual_grade: "90".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "93\n\n".to_string(),
                final_review: "92".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B3042288".to_string(),
                course_name: "Web应用系统开发".to_string(),
                course_credit: "2.5".to_string(),
                usual_grade: "100".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "79\n\n".to_string(),
                final_review: "85".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B4045109".to_string(),
                course_name: "Python基础".to_string(),
                course_credit: "2".to_string(),
                usual_grade: "92".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "87\n\n".to_string(),
                final_review: "90".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B704206".to_string(),
                course_name: "计算机拆装与维护实训".to_string(),
                course_credit: "1".to_string(),
                usual_grade: "94".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "73\n\n".to_string(),
                final_review: "86".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B7042659".to_string(),
                course_name: "软件工程实训".to_string(),
                course_credit: "3".to_string(),
                usual_grade: "".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "85\n\n".to_string(),
                final_review: "85".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
            CourseScore {
                course_code: "B7042664".to_string(),
                course_name: "Web应用系统开发课程设计".to_string(),
                course_credit: "2".to_string(),
                usual_grade: "".to_string(),
                mid_grade: "\n\n".to_string(),
                final_grade: "68\n\n".to_string(),
                final_review: "68".to_string(),
                second_final_grade: "\n\n".to_string(),
                second_final_review: "".to_string(),
            },
        ];
        assert_eq!(origin_course_score_vec, target_course_score_vec)
    }
}
