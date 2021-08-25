use zf_tools_rs::client::User;
use zf_tools_rs::parsers::{SchoolYear, Semester};
use zf_tools_rs::session::SessionBuilder;

#[tokio::main]
async fn main() {
    let mut session = SessionBuilder::new().user("user").passwd("passwd").build();
    let x = session.login().await;
    match x {
        Ok(mut y) => {
            let m = SchoolYear::SomeYear(2020);
            let timetable = y.get_timetable(m, Semester::SecondTerm).await;
            println!("{:?}", timetable);
        }
        _ => {
            println!("error")
        }
    }
}
