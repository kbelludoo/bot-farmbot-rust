use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: FarmBotSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(default)]
    pub cookie: String,
    #[serde(rename = "sessionToken", default)]
    pub session_token: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FarmBotSettings {
    #[serde(rename = "autoHunt", default = "default_true")]
    pub auto_hunt: bool,
    #[serde(rename = "targetLevelCode", default)]
    pub target_level_code: String,
    #[serde(rename = "autoAdvancePhase", default = "default_true")]
    pub auto_advance_phase: bool,
    #[serde(rename = "autoChests", default = "default_true")]
    pub auto_chests: bool,
    #[serde(rename = "autoMissions", default = "default_true")]
    pub auto_missions: bool,
    #[serde(rename = "autoTransmute", default)]
    pub auto_transmute: bool,
    #[serde(rename = "autoDismantle", default = "default_true")]
    pub auto_dismantle: bool,
    #[serde(rename = "speedMultiplier", default = "default_multiplier")]
    pub speed_multiplier: f64,
}

fn default_true() -> bool {
    true
}

fn default_multiplier() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionLeaseRequest {
    #[serde(rename = "holderId")]
    pub holder_id: String,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionLeaseResponse {
    #[serde(rename = "holderId")]
    pub holder_id: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountData {
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub gold: i64,
    #[serde(default)]
    pub gems: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActiveLevel {
    pub code: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BootstrapResponse {
    pub account: Option<AccountData>,
    #[serde(rename = "activeLevel")]
    pub active_level: Option<ActiveLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunStartRequest {
    #[serde(rename = "levelCode")]
    pub level_code: String,
    #[serde(rename = "clientRequestId")]
    pub client_request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunStartResponse {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(default)]
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunCompleteResponse {
    pub success: bool,
    #[serde(default)]
    pub xp: u64,
    #[serde(default)]
    pub gold: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChestItem {
    pub code: String,
    #[serde(default)]
    pub count: u32,
}
