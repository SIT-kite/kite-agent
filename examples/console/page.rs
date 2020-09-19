use crate::ConsoleResult;
use kite_agent::service::ActivityListRequest;
use kite_agent::service::CourseScoreRequest;
use kite_agent::service::ElectricityBillRequest;
use kite_agent::service::ActivityDetailRequest;
use kite_agent::service::ResponsePayload;
use kite_agent::{AgentData, SessionStorage};
use prettytable::{Cell, Row, Table};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Fetch various supported pages with the given account.
pub enum PageCommand {
    /// Query electricity bill.
    QueryElectricityBill(QueryElectricityBill),
    /// Query recent activities in second-course platform.
    GetRecentActivities(GetRecentActivities),
    /// Query score history.
    GetScoreList(GetScoreList),
    /// Get activity details.
    GetActivityDetail(GetActivityDetail),
}

impl PageCommand {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        match self {
            PageCommand::QueryElectricityBill(query) => query.process(sessions).await,
            PageCommand::GetRecentActivities(query) => query.process(sessions).await,
            PageCommand::GetScoreList(query) => query.process(sessions).await,
            PageCommand::GetActivityDetail(query) => query.process(sessions).await,
        }
    }
}

#[derive(StructOpt)]
pub struct QueryElectricityBill {
    #[structopt(long)]
    pub room: String,
}

impl QueryElectricityBill {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        println!("Query room {}", self.room);

        let mut table = Table::new();
        let request = ElectricityBillRequest { room: self.room };
        let response = request
            .process(AgentData {
                agent: "".to_string(),
                local_addr: "".to_string(),
                parameter: sessions.clone(),
            })
            .await?;

        if let ResponsePayload::ElectricityBill(data) = response {
            table.add_row(row!["ROOM", "BALANCE", "POWER AVAILABLE"]);

            table.add_row(Row::new(vec![
                Cell::new(&data.room),
                Cell::new(&data.total.to_string()),
                Cell::new(&data.power.to_string()),
            ]));
            table.printstd();
        }
        Ok(())
    }
}

#[derive(StructOpt)]
pub struct GetRecentActivities {
    /// Count of activities per page.
    #[structopt(long, short, default_value = "10")]
    pub count: u16,
    /// Page index.
    #[structopt(long, short, default_value = "1")]
    pub index: u16,
}

impl GetRecentActivities {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        let mut table = Table::new();
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
            .await?;

        if let ResponsePayload::ActivityList(activities) = response {
            table.add_row(row!["ID", "NAME"]);
            for activity in activities {
                table.add_row(Row::new(vec![
                    Cell::new(&activity.id),
                    Cell::new(&activity.title),
                ]));
            }
            table.printstd();
        }
        Ok(())
    }
}

#[derive(StructOpt)]
pub struct GetScoreList {
    #[structopt(long, short = "u")]
    pub account: String,
    #[structopt(long, short = "p")]
    pub credential: String,
    #[structopt(long, short)]
    pub term: String,
}

impl GetScoreList {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        let mut table = Table::new();
        let request = CourseScoreRequest {
            account: self.account,
            credential: self.credential,
            term: self.term,
        };

        let response = request
            .process(AgentData {
                agent: String::new(),
                local_addr: String::new(),
                parameter: sessions,
            })
            .await?;

        if let ResponsePayload::ScoreList(courses) = response {
            table.add_row(row!["ID", "NAME", "DETAIL"]);
            for course in courses {
                table.add_row(Row::new(vec![
                    Cell::new(&course.course_code),
                    Cell::new(&course.course_name),
                    Cell::new(&format!("{:?}", course.detail)),
                ]));
            }
            table.printstd();
        }
        Ok(())
    }
}


#[derive(StructOpt)]
pub struct GetActivityDetail {
    #[structopt(long)]
    pub id: String,
}

impl GetActivityDetail {
    pub async fn process(self, sessions: SessionStorage) -> ConsoleResult<()> {
        let request = ActivityDetailRequest {
            id: self.id,
        };

        let response = request
            .process(AgentData {
                agent: String::new(),
                local_addr: String::new(),
                parameter: sessions,
            })
            .await?;

        if let ResponsePayload::ActivityDetail(detail) = response {
            println!("{:#?}", detail);
        }
        Ok(())
    }
}

