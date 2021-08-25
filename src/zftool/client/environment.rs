use crate::zftool::client::ZfClient;
use crate::zftool::config::url::{CLASS_LIST, MAJOR_LIST, SUGGESTED_COURSE};
use crate::zftool::parsers::*;
use crate::zftool::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Environment {
    async fn get_major_list(&mut self, entrance_year: SchoolYear) -> Result<Vec<Major>>;

    async fn get_class_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Class>>;

    async fn get_suggested_course_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
        major_id: &str,
        class_id: &str,
        entrance_year: Option<&str>,
    ) -> Result<Vec<Course>>;
}

#[async_trait]
impl Environment for ZfClient {
    async fn get_major_list(&mut self, entrance_year: SchoolYear) -> Result<Vec<Major>> {
        let param = [("njdm_id", entrance_year.to_string())];
        let page = self.get_url(MAJOR_LIST, &param).await?;
        let text = page.text().await?;
        parse_major_list_page(&text)
    }

    async fn get_class_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Class>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("queryModel.showCount", 10000.to_string()),
        ];
        let page = self.post_url(CLASS_LIST, &data).await?;
        let text = page.text().await?;
        parse_class_list_page(&text)
    }

    async fn get_suggested_course_list(
        &mut self,
        school_year: SchoolYear,
        semester: Semester,
        major_id: &str,
        class_id: &str,
        entrance_year: Option<&str>,
    ) -> Result<Vec<Course>> {
        let mut year;
        match entrance_year {
            Some(x) => {
                year = x.to_string();
            }
            None => {
                year = "20".to_string();
                let classes = class_id.chars();
                let mut count = 0;
                for text in classes {
                    year += &text.to_string();
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("njdm_id", year),
            ("zyh_id", major_id.to_string()),
            ("bh_id", class_id.to_string()),
            ("tjkbzdm", "1".to_string()),
            ("tjkbzxsdm", "0".to_string()),
        ];
        let page = self.post_url(SUGGESTED_COURSE, &data).await?;
        let text = page.text().await?;
        parse_timetable_page(&text)
    }
}
