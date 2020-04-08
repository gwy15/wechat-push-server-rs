use log;
use serde_derive::Deserialize;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to open file '{}' due to IO error", file)]
    Io { file: String },

    #[fail(display = "Failed to parse file '{}' as toml", file)]
    Toml { file: String },

    #[fail(
        display = "Failed to parse toml value from file '{}' as config object",
        file
    )]
    Convert { file: String },
}

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
    pub wechat: WechatConfig,
}

impl Config {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Config {
            root_url: "".to_owned(),
            redis_url: "127.0.0.1:6379".to_owned(),
            wechat: WechatConfig {
                app_id: "app_id".to_owned(),
                app_secret: "app_secret".to_owned(),
                token: "token".to_owned(),
            },
        }
    }

    fn merge(mut self, update: Self) -> Self {
        self.root_url = update.root_url;
        self.redis_url = update.redis_url;
        // wechat
        self.wechat.app_id = update.wechat.app_id;
        self.wechat.app_secret = update.wechat.app_secret;
        self.wechat.token = update.wechat.token;

        // return
        self
    }

    fn from_filename(filename: &str) -> Result<Self, Error> {
        let s = std::fs::read_to_string(filename).map_err(|_| Error::Io {
            file: filename.to_owned(),
        })?;
        let value: toml::Value = toml::from_str(&s).map_err(|_| Error::Toml {
            file: filename.to_owned(),
        })?;
        value
            .try_into::<Self>()
            .map_err(|_| {
                log::warn!("Failed to load config file {}", filename);
                Error::Convert {
                    file: filename.to_owned(),
                }
            })
            .and_then(|c| {
                log::debug!("Config file {} loaded.", filename);
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

    pub fn load(testing: bool) -> Result<Self, Error> {
        // load by order: config.toml, config.local.toml
        // if testing is true, load config.testing.toml as well
        let mut config = Self::from_filename("config.toml")?;

        // allow config.local.toml missing
        config = match Self::from_filename("config.local.toml") {
            Err(_) => config,
            Ok(update) => config.merge(update),
        };

        if testing {
            config = match Self::from_filename("config.testing.toml") {
                Err(_) => config,
                Ok(update) => config.merge(update),
            }
        }

        // check
        config.check();

        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_not_existing_file() {
        let result = Config::from_filename("not exist");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_ok() {
        assert!(Config::from_filename("config.toml").is_ok());
    }

    #[test]
    fn test_config_check() {
        let mut config = Config::default();
        // "/"
        config.root_url = "/".to_owned();
        config.check();
        assert_eq!(config.root_url, "");
        // "/api/"
        config.root_url = "/api/".to_owned();
        config.check();
        assert_eq!(config.root_url, "/api");
    }
}
