use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::api::FarmClient;
use crate::game::types::*;
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub api: Arc<FarmClient>,

    pub account_data: Arc<Mutex<AccountData>>,
    pub active_level: Arc<Mutex<String>>,
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub running: AtomicBool,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));
        let api = Arc::new(FarmClient::new(&config));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            api,
            account_data: Arc::new(Mutex::new(AccountData::default())),
            active_level: Arc::new(Mutex::new("level-1".to_string())),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(1),
            running: AtomicBool::new(false),
        }
    }

    pub async fn init(&self) -> Result<(), String> {
        info!("Verificando bootstrap e sessão no Farm Ricochet...");
        let holder_id = uuid::Uuid::new_v4().to_string();
        let _ = self.api.acquire_session_lease(&holder_id).await;

        if let Ok(boot) = self.api.get_bootstrap().await {
            if let Some(acc) = boot.account {
                self.current_level.store(acc.level, Ordering::Relaxed);
                self.gold.store(acc.gold as u64, Ordering::Relaxed);
                *self.account_data.lock().await = acc;
            }
            if let Some(lvl) = boot.active_level {
                *self.active_level.lock().await = lvl.code;
            }
        }

        Ok(())
    }

    pub async fn run_campaign_step(&self) {
        let level_code = self.active_level.lock().await.clone();
        info!("🏹 [Campanha] Iniciando fase '{level_code}'...");

        match self.api.start_run(&level_code).await {
            Ok(run_start) => {
                let dur = Duration::from_millis((4000.0 * self.config.settings.speed_multiplier) as u64);
                sleep(dur).await;

                match self.api.complete_run(&run_start.run_id).await {
                    Ok(resp) => {
                        if resp.success {
                            info!("🎉 Fase '{level_code}' concluída com sucesso! +{} XP, +{} Gold", resp.xp, resp.gold);
                            self.kills.fetch_add(15, Ordering::Relaxed);
                            self.gold.fetch_add(resp.gold, Ordering::Relaxed);
                            self.exp.fetch_add(resp.xp, Ordering::Relaxed);
                        }
                    }
                    Err(e) => warn!("Falha ao completar run: {e}"),
                }
            }
            Err(e) => warn!("Falha ao iniciar run: {e}"),
        }
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (Farm Ricochet) iniciado com sucesso.", self.config.name);

        let holder_id = uuid::Uuid::new_v4().to_string();
        let _ = self.api.acquire_session_lease(&holder_id).await;

        let mut heartbeat_timer = tokio::time::interval(Duration::from_secs(30));
        let mut campaign_timer = tokio::time::interval(Duration::from_secs(6));
        let mut loot_timer = tokio::time::interval(Duration::from_secs(60));

        while self.running.load(Ordering::Relaxed) {
            tokio::select! {
                _ = heartbeat_timer.tick() => {
                    let _ = self.api.heartbeat_session_lease(&holder_id).await;
                }
                _ = campaign_timer.tick() => {
                    if self.config.settings.auto_hunt {
                        self.run_campaign_step().await;
                    }
                }
                _ = loot_timer.tick() => {
                    let _ = self.api.claim_run_loot().await;
                    if self.config.settings.auto_chests {
                        let _ = self.api.open_chest("free-daily").await;
                    }
                }
            }
        }
    }
}
