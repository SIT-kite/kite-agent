//! This module provides ability to test network connectivity and try to connect the campus network.

use rand::{seq::SliceRandom, thread_rng};

use super::user_agent;
use crate::error::Result;
use std::collections::HashMap;

pub struct NetworkTestPage {
    pub url: &'static str,
    pub expected_response: &'static str,
}

/* Defualt configuration */
lazy_static! {
    /// Test page provided by the public service which are always available and return 200 OK.
    pub static ref TEST_PAGES: Vec<NetworkTestPage> = vec![
        NetworkTestPage {
            url: "http://www.msftconnecttest.com/connecttest.txt",
            expected_response: "Microsoft Connect Test",
        },
        NetworkTestPage {
            url: "http://captive.apple.com/hotspot-detect.html",
            expected_response: "Success",
        },
        NetworkTestPage {
            url: "http://detectportal.firefox.com/",
            expected_response: "success",
        },
    ];
}

/// Campus network portal address.
pub const PORTAL_ADDRESS: &str = "http://172.16.8.70";

/// Get a random test url and its expected response.
fn get_test_page() -> &'static NetworkTestPage {
    let mut rng = thread_rng();

    return TEST_PAGES[..].choose(&mut rng).unwrap();
}

/// Network connectivity status.
#[derive(Debug)]
pub enum NetworkConnectivity {
    /// Can be used normally.
    Connected,
    /// Connected, but need more actions.
    LoginNeeded,
    /// Do not connect to server.
    NoConnection,
}

/// Test network connectivity.
/// See `NetworkConnectivity` enum details.
pub async fn test_network_connectivity() -> NetworkConnectivity {
    let test_page = get_test_page();

    // Create request builder and send request
    let response = reqwest::Client::new()
        .get(test_page.url)
        .header("User-Agent", user_agent::get_random_ua_string().as_str())
        .send()
        .await;

    match response {
        Ok(r) => {
            // Get response body and differ it from expected_response.
            if r.status().is_success()
                && r.text().await.unwrap_or_default() == test_page.expected_response
            {
                return NetworkConnectivity::Connected;
            }
            NetworkConnectivity::LoginNeeded
        }
        Err(_) => NetworkConnectivity::NoConnection,
    }
}

/// Send login request to portal server
pub async fn connect_campus_network(student_id: &str, password: &str) -> Result<()> {
    let mut post_data = HashMap::new();
    post_data.insert("DDDD", student_id);
    post_data.insert("upass", password);
    post_data.insert("0MKKey", "%B5%C7%A1%A1%C2%BC");

    let _response = reqwest::Client::new()
        .post(&format!("{}/0.htm", PORTAL_ADDRESS))
        .form(&post_data)
        .send()
        .await?;
    // if response.status().is_success() {
    //     return Err(AgentError::Http(response.status()));
    // }
    Ok(())
}
