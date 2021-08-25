use crate::zftool::client::ZfClient;
use crate::zftool::config::url::{PROFILE, SCORE_LIST, TIME_TABLE};
use crate::zftool::config::USERAGENT;
use crate::zftool::parsers::*;
use crate::zftool::Result;
use async_trait::async_trait;
use reqwest::header::{COOKIE, USER_AGENT};
use std::collections::HashMap;

#[async_trait]
pub trait User {
    async fn get_profile(&mut self) -> Result<Profile>;

    async fn get_timetable(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Course>>;

    fn group_timetable(course_list: Vec<Course>) -> HashMap<String, Vec<Course>>;

    async fn get_group_timetable(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<HashMap<String, Vec<Course>>>;

    async fn get_score_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Score>>;

    fn calculate_gpa(score_list: Vec<Score>) -> Result<f32>;

    async fn get_gpa(&mut self, school_year: SchoolYear, semester: Semester) -> Result<f32>;
}

#[async_trait]
impl User for ZfClient {
    async fn get_profile(&mut self) -> Result<Profile> {
        let page = self
            .session
            .client
            .get(PROFILE)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        self.session.sync_cookies("jwxt.sit.edu.cn", page.cookies());
        let text = page.text().await?;
        parse_profile_page(&text)
    }

    async fn get_timetable(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Course>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
        ];
        let page = self.post_url(TIME_TABLE, &data).await?;
        let text = page.text().await?;
        parse_timetable_page(&text)
    }

    fn group_timetable(course_list: Vec<Course>) -> HashMap<String, Vec<Course>> {
        let mut result: HashMap<String, Vec<Course>> = HashMap::new();
        for course in course_list {
            let course_name = course.course_name.clone();
            if result.contains_key(&course_name) {
                let mut v = result.remove(&course_name).unwrap();
                v.push(course.clone());
                result.insert(course_name, v);
            } else {
                result.insert(course_name, vec![course]);
            }
        }
        result
    }

    async fn get_group_timetable(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<HashMap<String, Vec<Course>>> {
        let time_table = self.get_timetable(school_year, semester).await?;
        return Ok(ZfClient::group_timetable(time_table));
    }

    async fn get_score_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Score>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("queryModel.showCount", "5000".to_string()),
        ];
        let page = self.post_url(SCORE_LIST, &data).await?;
        let text = page.text().await?;
        parse_score_list_page(&text)
    }

    fn calculate_gpa(score_list: Vec<Score>) -> Result<f32> {
        Ok(calculate_gpa(score_list))
    }

    async fn get_gpa(&mut self, school_year: SchoolYear, semester: Semester) -> Result<f32> {
        let score_list = self.get_score_list(school_year, semester).await?;
        return ZfClient::calculate_gpa(score_list);
    }
}
