use zf_tools_rs::client::Environment;
use zf_tools_rs::parsers::{SchoolYear, Semester};
use zf_tools_rs::session::SessionBuilder;

#[tokio::main]
async fn main() {
    let mut session = SessionBuilder::new().user("user").passwd("passwd").build();
    let x = session.login().await;
    match x {
        Ok(mut y) => {
            let m = SchoolYear::SomeYear(2018);
            let major = y.get_class_list(m, Semester::SecondTerm).await;
            println!("{:?}", major);
        }
        _ => {
            println!("error")
        }
    }
}
