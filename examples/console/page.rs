use kite_agent::service::ElectricityBillRequest;
use kite_agent::{AgentData, SessionStorage};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Fetch various supported pages with the given account.
pub enum PageCommand {
    /// Query electricity bill.
    QueryElectricityBill(QueryElectricityBill),
}

impl PageCommand {
    pub async fn process(self, sessions: SessionStorage) {
        match self {
            PageCommand::QueryElectricityBill(query) => query.process(sessions).await,
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
