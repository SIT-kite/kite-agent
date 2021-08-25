use zf_tools_rs::client::Environment;
use zf_tools_rs::parsers::SchoolYear;
use zf_tools_rs::session::SessionBuilder;

#[tokio::main]
async fn main() {
    let mut session = SessionBuilder::new().user("user").passwd("passwd").build();
    let x = session.login().await;
    match x {
        Ok(mut y) => {
            let m = SchoolYear::SomeYear(2018);
            let major = y.get_major_list(m).await;
            println!("{:?}", major);
        }
        _ => {
            println!("error")
        }
    }
}
