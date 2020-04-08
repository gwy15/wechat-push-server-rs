use std::sync::Mutex;

use redis::Client as RedisClient;

use crate::config::Config;
use crate::errors::Error;
use crate::wechat::TokenManager;

pub struct AppState {
    pub token_manager: Mutex<TokenManager>,
    pub redis: RedisClient,
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let token_manager = TokenManager::new(
            config.wechat.app_id.clone(),
            config.wechat.app_secret.clone(),
        );
        let redis_client = RedisClient::open(config.redis_url.clone()).unwrap();
        AppState {
            token_manager: Mutex::new(token_manager),
            redis: redis_client,
            config,
        }
    }

    pub async fn redis_connection(&self) -> Result<redis::aio::Connection, Error> {
        Ok(self.redis.get_async_connection().await?)
    }
}
