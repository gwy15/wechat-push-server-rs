use config::{Config as ConfigMod, ConfigError, File};
use log;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WechatConfig {
    pub app_id: String,
    pub app_secret: String,
    pub token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub root_url: String,
    pub redis_url: String,
    pub postgres_url: String,
    pub wechat: WechatConfig,
}

impl Config {
    pub fn new(testing: bool) -> Result<Self, ConfigError> {
        let mut s = ConfigMod::new();
        // default
        s.merge(File::with_name("config.toml"))?;
        // local
        s.merge(File::with_name("config.local").required(false))?;
        // testing
        if testing {
            s.merge(File::with_name("config.testing"))?;
        }

        s.try_into().and_then(|mut c: Self| {
            c.check();
            log::info!("Config loaded.");
            Ok(c)
        })
    }

    /// Check config validation, trims url, etc.
    fn check(&mut self) {
        if let Some('/') = self.root_url.chars().rev().next() {
            self.root_url = self
                .root_url
                .chars()
                .take(self.root_url.len() - 1)
                .collect();
        }
    }
}
