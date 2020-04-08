use log;
use serde_json::json;
use serde_json::Value;
use std::time::{Duration, SystemTime};

use super::errors::WechatError;
use super::Request;

type TimeStamp = SystemTime;

#[derive(Clone)]
pub struct Token {
    token: String,
    expires: TimeStamp,
}

impl Token {
    pub fn new(token: String, expires: TimeStamp) -> Token {
        Token { token, expires }
    }

    pub fn expired(&self) -> bool {
        SystemTime::now() > self.expires
    }

    pub fn string(self) -> String {
        self.token
    }
}

pub struct TokenManager {
    app_id: String,
    app_secret: String,
    token: Option<Token>,
}

impl TokenManager {
    pub fn new(app_id: String, app_secret: String) -> Self {
        TokenManager {
            app_id,
            app_secret,
            token: None,
        }
    }

    /// Get access token, apply one if current one is no good
    pub async fn get_access_token<'a>(&'a mut self) -> Result<Token, WechatError> {
        match &self.token {
            None => self.apply_new_token().await,
            Some(t) => match t.expired() {
                true => self.apply_new_token().await,
                false => Ok(t.clone()),
            },
        }
    }

    /// Apply for a new token from wechat server
    async fn apply_new_token(&mut self) -> Result<Token, WechatError> {
        log::debug!("Applying for a new access token");

        const URL: &'static str = "https://api.weixin.qq.com/cgi-bin/token";
        let params = json!({
            "grant_type": "client_credential",
            "appid": self.app_id.as_str(),
            "secret": self.app_secret.as_str(),
        });

        // form request
        let request = Request::get(URL).data(&params);
        //
        let data = request.send().await?;

        TokenManager::parse_response_body(data).map(|token| {
            self.token = Some(token.clone());
            log::info!("A new wechat token was successfully issued.");
            token
        })
    }

    fn parse_response_body(data: Value) -> Result<Token, WechatError> {
        use super::errors::GetKey;
        let access_token = data.get_key("access_token")?;
        let duration = data.get_key("expires_in")?;
        let expires_in = SystemTime::now() + Duration::from_secs(duration);
        Ok(Token::new(access_token, expires_in))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_token_expire() {
        // not expired
        let t = SystemTime::now() + Duration::from_secs(1);
        let token = Token::new("123".to_owned(), t);
        assert!(!token.expired());
        // expired
        let t = SystemTime::now() - Duration::from_secs(1);
        let token = Token::new("123".to_owned(), t);
        assert!(token.expired());
    }

    #[test]
    fn test_data_parse() {
        use serde_json::json;
        let res = TokenManager::parse_response_body(json! ({
            "access_token": "123",
            "expires_in": 123
        }));
        assert!(res.is_ok());
        let token = res.unwrap();
        assert!(!token.expired());
    }
}
