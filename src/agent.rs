use kite_protocol::agent::AgentBuilder;
use kite_protocol::error::Result;
use kite_protocol::services::Body;
use std::sync::Arc;
use tokio::time::Duration;

fn router(request_body: Body) -> Result<Body> {
    return match request_body {
        Body::Heartbeat(heartbeat) => Ok(Body::Heartbeat(heartbeat.pong())),
        _ => Ok(Body::Empty),
    };
}

async fn agent_main() {
    let mut agent = AgentBuilder::new(String::from("agent01"), 8910)
        .host("127.0.0.1", 8288)
        .set_heartbeart_interval(Duration::from_secs(30))
        .set_callback(Arc::new(router))
        .build();

    agent.start();

    loop {
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}
