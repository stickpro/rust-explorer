use std::sync::Arc;
use tokio::sync::Notify;
use crate::client::bitcoin::{BitcoinClient, BitcoinClientExt};
use crate::client::database::{DatabaseClient, DatabaseClientExt};
use crate::configure::AppConfig;
use crate::error::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
    pub bitcoin: Arc<BitcoinClient>,
    pub messenger_notify: Arc<Notify>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
        let bitcoin = Arc::new(BitcoinClient::build_from_config(&config.bitcoin).await?);
        Ok(Self {
            config: Arc::new(config),
            db,
            bitcoin,
            messenger_notify: Default::default(),
        })
    }
}