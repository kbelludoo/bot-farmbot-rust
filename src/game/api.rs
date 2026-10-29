use crate::game::types::*;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, ORIGIN, REFERER, USER_AGENT};
use std::time::Duration;
use tracing::{debug, error, info};

pub struct FarmClient {
    client: reqwest::Client,
    base_url: String,
}

impl FarmClient {
    pub fn new(config: &AccountConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(ORIGIN, HeaderValue::from_static("https://farmricochet.com"));
        headers.insert(REFERER, HeaderValue::from_static("https://farmricochet.com/play"));

        if !config.credentials.cookie.is_empty() {
            if let Ok(val) = HeaderValue::from_str(&config.credentials.cookie) {
                headers.insert(COOKIE, val);
            }
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(20))
            .build()
            .expect("Farm HTTP Client");

        let base_url = if config.credentials.origin.is_empty() {
            "https://api.farmricochet.com".to_string()
        } else {
            config.credentials.origin.trim_end_matches('/').to_string()
        };

        Self { client, base_url }
    }

    pub async fn acquire_session_lease(&self, holder_id: &str) -> Result<SessionLeaseResponse, String> {
        let url = format!("{}/api/v1/game-session", self.base_url);
        let req = SessionLeaseRequest {
            holder_id: holder_id.to_string(),
            force: true,
        };
        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Lease error: {e}"))?;

        resp.json::<SessionLeaseResponse>()
            .await
            .map_err(|e| format!("Lease decode error: {e}"))
    }

    pub async fn heartbeat_session_lease(&self, holder_id: &str) -> Result<bool, String> {
        let url = format!("{}/api/v1/game-session", self.base_url);
        let req = SessionLeaseRequest {
            holder_id: holder_id.to_string(),
            force: false,
        };
        let resp = self
            .client
            .put(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Lease heartbeat error: {e}"))?;

        Ok(resp.status().is_success())
    }

    pub async fn get_bootstrap(&self) -> Result<BootstrapResponse, String> {
        let url = format!("{}/api/v1/bootstrap?mapImages=cdn", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Bootstrap error: {e}"))?;

        resp.json::<BootstrapResponse>()
            .await
            .map_err(|e| format!("Bootstrap decode error: {e}"))
    }

    pub async fn start_run(&self, level_code: &str) -> Result<RunStartResponse, String> {
        let url = format!("{}/api/v1/runs/start", self.base_url);
        let req = RunStartRequest {
            level_code: level_code.to_string(),
            client_request_id: uuid::Uuid::new_v4().to_string(),
        };
        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Start run error: {e}"))?;

        resp.json::<RunStartResponse>()
            .await
            .map_err(|e| format!("Start run decode error: {e}"))
    }

    pub async fn complete_run(&self, run_id: &str) -> Result<RunCompleteResponse, String> {
        let url = format!("{}/api/v1/runs/{}/complete", self.base_url, run_id);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "durationMs": 4500,
                "result": "victory"
            }))
            .send()
            .await
            .map_err(|e| format!("Complete run error: {e}"))?;

        resp.json::<RunCompleteResponse>()
            .await
            .map_err(|e| format!("Complete run decode error: {e}"))
    }

    pub async fn claim_run_loot(&self) -> Result<bool, String> {
        let url = format!("{}/api/v1/run-loot/claim", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "idempotencyKey": uuid::Uuid::new_v4().to_string()
            }))
            .send()
            .await
            .map_err(|e| format!("Claim loot error: {e}"))?;

        Ok(resp.status().is_success())
    }

    pub async fn open_chest(&self, code: &str) -> Result<bool, String> {
        let url = format!("{}/api/v1/chests/{}/open", self.base_url, code);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "clientRequestId": uuid::Uuid::new_v4().to_string()
            }))
            .send()
            .await
            .map_err(|e| format!("Open chest error: {e}"))?;

        Ok(resp.status().is_success())
    }
}
