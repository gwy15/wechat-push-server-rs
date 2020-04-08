use std::sync::Mutex;

// redis
use redis::Client as RedisClient;
// SQL (postgres)
use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::config::Config;
use crate::errors::Error;
use crate::wechat::TokenManager;

pub struct AppState {
    pub token_manager: Mutex<TokenManager>,
    pub redis: RedisClient,
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,

    pub config: Config,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        let token_manager = TokenManager::new(
            config.wechat.app_id.clone(),
            config.wechat.app_secret.clone(),
        );
        // redis
        let redis_client = RedisClient::open(config.redis_url.clone()).unwrap();
        // sql
        let db_manager = ConnectionManager::<PgConnection>::new(&config.postgres_url);
        let db_pool = r2d2::Pool::builder()
            .build(db_manager)
            .expect("Failed to create Postgres pool.");
        //

        // return app state
        AppState {
            token_manager: Mutex::new(token_manager),
            redis: redis_client,
            db_pool,
            config,
        }
    }

    pub async fn redis_connection(&self) -> Result<redis::aio::Connection, Error> {
        Ok(self.redis.get_async_connection().await?)
    }
}
