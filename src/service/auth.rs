use crate::agent::SharedData;
use crate::net::auth::portal_login;
use crate::net::Session;
use crate::service::{ActionError, ResponsePayload, ResponseResult};

use super::DoRequest;

#[derive(Debug, serde::Deserialize)]
pub struct PortalAuthRequest {
    account: String,
    credential: String,
}

#[derive(Debug, serde::Serialize)]
pub enum PortalAuthResponse {
    Ok,
    Err(String),
}

#[async_trait::async_trait]
impl DoRequest for PortalAuthRequest {
    async fn process(self, mut data: SharedData) -> ResponseResult {
        let session = portal_login(&data.client, &self.account, &self.credential).await?;

        data.session_store.insert(&session)?;
        Ok(ResponsePayload::PortalAuth(PortalAuthResponse::Ok))
    }
}
