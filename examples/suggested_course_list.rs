use zf_tools_rs::client::Environment;
use zf_tools_rs::parsers::{SchoolYear, Semester};
use zf_tools_rs::session::SessionBuilder;

#[tokio::main]
async fn main() {
    let mut session = SessionBuilder::new().user("user").passwd("passwd").build();
    let x = session.login().await;
    match x {
        Ok(mut y) => {
            let m = SchoolYear::SomeYear(2020);
            let course = y
                .get_suggested_course_list(
                    m,
                    Semester::SecondTerm,
                    "A79D525F036EC7D2E055E2219BB20201",
                    "191042Y1",
                    None,
                )
                .await;
            println!("{:?}", course);
        }
        _ => {
            println!("error")
        }
    }
}
