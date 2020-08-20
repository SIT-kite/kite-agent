use kite_agent::service::ActivityListRequest;
use kite_agent::service::ElectricityBillRequest;
use kite_agent::{AgentData, SessionStorage};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Fetch various supported pages with the given account.
pub enum PageCommand {
    /// Query electricity bill.
    QueryElectricityBill(QueryElectricityBill),
    /// Query recent activities in second-course platform.
    GetRecentActivities(GetRecentActivities),
}

impl PageCommand {
    pub async fn process(self, sessions: SessionStorage) {
        match self {
            PageCommand::QueryElectricityBill(query) => query.process(sessions).await,
            PageCommand::GetRecentActivities(query) => query.process(sessions).await,
        }
    }
}

#[derive(StructOpt)]
pub struct QueryElectricityBill {
    #[structopt(long)]
    pub room: String,
}

impl QueryElectricityBill {
    pub async fn process(self, sessions: SessionStorage) {
        println!("Query room {}", self.room);

        let request = ElectricityBillRequest { room: self.room };
        let response = request
            .process(AgentData {
                agent: "".to_string(),
                local_addr: "".to_string(),
                parameter: sessions.clone(),
            })
            .await;

        println!("{:?}", response);
    }
}

#[derive(StructOpt)]
pub struct GetRecentActivities {
    // #[structopt(long)]
    // pub account: Option<String>,
    // #[structopt(long)]
    // pub credential: Option<String>,
    /// Count of activities per page.
    #[structopt(long, short, default_value = "10")]
    pub count: u16,
    /// Page index.
    #[structopt(long, short, default_value = "1")]
    pub index: u16,
}

impl GetRecentActivities {
    pub async fn process(self, sessions: SessionStorage) {
        let request = ActivityListRequest {
            count: self.count,
            index: self.index,
        };
        let response = request
            .process(AgentData {
                agent: "".to_string(),
                local_addr: "".to_string(),
                parameter: sessions.clone(),
            })
            .await;
    }
}
