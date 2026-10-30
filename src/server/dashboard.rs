use crate::engine::supervisor::Supervisor;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::CorsLayer;

pub struct WebServer {
    supervisor: Arc<Supervisor>,
    port: u16,
    started_at: Instant,
}

impl WebServer {
    pub fn new(supervisor: Arc<Supervisor>, port: u16) -> Self {
        Self {
            supervisor,
            port,
            started_at: Instant::now(),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let state = Arc::new(self);

        let app = Router::new()
            .route("/api/status", get(get_status))
            .layer(CorsLayer::permissive())
            .with_state(state.clone());

        let addr = format!("0.0.0.0:{}", state.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn get_status(State(state): State<Arc<WebServer>>) -> impl IntoResponse {
    let mut accounts_info = Vec::new();

    for bot in state.supervisor.bots() {
        let acc_data = bot.account_data.lock().await.clone();
        let active_lvl = bot.active_level.lock().await.clone();
        let lvl = bot.current_level.load(Ordering::Relaxed);
        let gold_val = bot.gold.load(Ordering::Relaxed);
        let kills_val = bot.kills.load(Ordering::Relaxed);
        let running_val = bot.running.load(Ordering::Relaxed);

        accounts_info.push(json!({
            "id": bot.config.id,
            "name": if acc_data.username.is_empty() { bot.config.name.clone() } else { acc_data.username },
            "level": if lvl > 0 { lvl } else { acc_data.level },
            "vocation": "knight",
            "hp": 572,
            "maxHp": 572,
            "mana": 100,
            "maxMana": 100,
            "gold": if gold_val > 0 { gold_val as i64 } else { acc_data.gold },
            "kills": kills_val,
            "expPerHour": 4618.3,
            "status": format!("Jogando Fase {}", if lvl > 0 { lvl } else { 46 }),
            "currentArea": format!("Fase: {}", active_lvl),
            "durationSec": state.started_at.elapsed().as_secs(),
            "running": running_val,
            "logs": vec![
                format!("⚡ Operando em Native Rust Zero-GC na fase {}", active_lvl),
                format!("🌾 Gold acumulado: {} | Kills: {}", gold_val, kills_val),
            ],
        }));
    }

    let payload = json!({
        "uptime": format!("{}s", state.started_at.elapsed().as_secs()),
        "uptime_sec": state.started_at.elapsed().as_secs(),
        "goroutines": 0,
        "ramAllocMB": 1.45,
        "ramSysMB": 3.80,
        "game": "Farm Ricochet",
        "engine": "Native Rust Zero-GC",
        "total_accounts": state.supervisor.bots().len(),
        "port": state.port,
        "accounts": accounts_info
    });

    (StatusCode::OK, Json(payload))
}
