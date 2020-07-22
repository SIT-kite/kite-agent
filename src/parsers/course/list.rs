use super::Parse;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

/// Course time arrangement
#[derive(Debug, Clone, PartialEq)]
pub struct CourseTime {
    /// 如 (1, 18) 表示 第 1..18 周
    pub week_range: (u8, u8),
    /// 0 as every week, 1 as odd weeks, 2 as even weeks
    pub week_type: u8,
    /// Day index in one week, like 周 1
    pub day_index: u8,
    /// Class and end index
    pub day_range: (u8, u8),
}

/// Course details
#[derive(Debug, Clone, PartialEq)]
pub struct CourseDetail {
    /// Course unique identifier.
    pub code: String,
    /// Course name.
    pub name: String,
    /// The course unique identifier in the semester.
    pub id: String,
    /// Course type.
    pub ctype: String,
    /// Course credit.
    pub credit: f32,
    /// Teacher name(May be more than one teacher)
    pub teahcers: Vec<String>,
    /// Class schedule
    pub schedule: Vec<CourseTime>,
    /// Classroom place
    pub place: Vec<String>,
    /// Campus
    pub campus: String,
    /// Planned (or max) people count in class
    pub planned_count: u16,
    /// Selected people count
    pub selected_count: u16,
    /// Prearranged class
    pub arranged_class: Vec<String>,
    /// Note
    pub note: String,
}

fn parse_range_string(range_string: &String) -> (u8, u8) {
    let l: Vec<&str> = range_string.split("-").collect();
    if l.len() == 2 {
        return (l[0].parse().unwrap_or_default(), l[1].parse().unwrap_or_default());
    }
    return (l[0].parse().unwrap_or_default(), l[0].parse().unwrap_or_default());
}

fn parse_time_string(time_string: &String) -> Vec<CourseTime> {
    let time_string = time_string.replace("<br>", "");
    let time_array: Vec<&str> = time_string.split(";").collect();
    let time_regex = Regex::new(r"第(\d+(?:-\d+)?)周([*]{0,2}),周(\d),第(\d+(?:-\d+)?)节").unwrap();
    let mut result = Vec::<CourseTime>::new();

    for each_time in time_array {
        time_regex.captures_iter(each_time).for_each(|items| {
            result.push(CourseTime {
                week_range: parse_range_string(&items[1].to_string()),
                week_type: items[2].len() as u8,
                day_index: items[3].parse().unwrap_or_default(),
                day_range: parse_range_string(&items[4].to_string()),
            });
        });
    }
    result
}

impl CourseDetail {
    fn from(fields: ElementRef, selector: &Selector) -> Self {
        let cols = fields
            .select(&selector)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>();

        Self {
            code: cols[2].to_string(),
            name: cols[1].to_string(),
            id: cols[0].to_string(),
            ctype: cols[3].to_string(),
            credit: cols[4].parse().unwrap_or_default(),
            teahcers: cols[5].split(",").map(|x| x.to_string()).collect(),
            schedule: parse_time_string(&cols[6]),
            place: cols[7].split(",").map(|x| x.to_string()).collect(),
            campus: cols[8].to_string(),
            planned_count: cols[9].parse().unwrap_or_default(),
            selected_count: cols[10].parse().unwrap_or_default(),
            arranged_class: cols[12].split(", ").map(|x| x.to_string()).collect(),
            note: cols[13].to_string(),
        }
    }
}

impl Parse for Vec<CourseDetail> {
    fn from_html(html_page: &str) -> Self {
        // Read html page to parser.
        let document = Html::parse_document(html_page);
        let selector = Selector::parse("table > tbody > tr[bgcolor=\"#efefef\"]").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        // Map each line to CourseDetail structure3159
        let courses = document
            .select(&selector)
            .map(|x| CourseDetail::from(x, &td_selector))
            .collect();
        courses
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_course_detail_parser() {
        use super::Parse;
        use super::{CourseDetail, CourseTime};

        let content = std::fs::read_to_string("html/课程列表页面 UTF8.html").unwrap();
        let courses: Vec<CourseDetail> = Parse::from_html(content.as_ref());

        assert_eq!(courses.len(), 3159);
        assert_eq!(
            courses[0],
            CourseDetail {
                code: String::from("B1040123"),
                name: String::from("计算机基础"),
                id: String::from("1800015"),
                ctype: String::from("公共基础课"),
                credit: 2.0,
                teahcers: vec![String::from("姜寒")],
                schedule: vec![
                    CourseTime {
                        week_range: (4, 11),
                        week_type: 0,
                        day_index: 1,
                        day_range: (1, 2)
                    },
                    CourseTime {
                        week_range: (3, 3),
                        week_type: 0,
                        day_index: 1,
                        day_range: (1, 2)
                    },
                    CourseTime {
                        week_range: (3, 12),
                        week_type: 0,
                        day_index: 5,
                        day_range: (3, 4)
                    },
                    CourseTime {
                        week_range: (12, 12),
                        week_type: 0,
                        day_index: 1,
                        day_range: (1, 2)
                    }
                ],
                place: vec![
                    String::from("奉计15（三教407）"),
                    String::from("一教A412(艺术特教专用）")
                ],
                campus: String::from("奉贤校区"),
                planned_count: 15,
                selected_count: 15,
                arranged_class: vec![String::from("18109331")],
                note: String::from("")
            }
        )
    }
}
