use config::{Config as ConfigMod, ConfigError, File};
use log;
use serde_derive::Deserialize;
use std::env;

#[derive(Deserialize, Debug, Clone)]
pub struct WechatConfig {
    pub app_id: String,
    pub app_secret: String,
    pub token: String,
    pub default_template_id: String,
    pub detail_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub root_url: String,
    pub redis_url: String,
    pub postgres_url: String,
    pub wechat: WechatConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = ConfigMod::new();
        // default
        s.merge(File::with_name("config/config.default.toml"))?;
        s.merge(File::with_name("config/config.local.toml").required(false))?;
        // Am I in CI?
        let workflow_name = env::var("GITHUB_WORKFLOW").unwrap_or("".into());
        if workflow_name != "" {
            log::info!("CI environment detected: {}", workflow_name);
            s.merge(File::with_name("config/config.ci.toml"))?;
            s.merge(File::with_name("config/config.ci.local.toml").required(false))?;
        }

        s.try_into().and_then(|mut c: Self| {
            c.check();
            log::info!("Config loaded.");
            Ok(c)
        })
    }

    /// Check config validation, trims url, etc.
    fn check(&mut self) {
        self.root_url = self.root_url.trim_end_matches("/").to_owned();
        self.wechat.detail_url = self.wechat.detail_url.trim_end_matches("/").to_owned();
    }
}
