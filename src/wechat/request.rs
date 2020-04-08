use reqwest::{Client, RequestBuilder};
use serde_json::json;
use serde_json::Value;

use super::errors::WechatError;
use super::TokenManager;

enum RequestType {
    Get,
    Post,
}

pub struct Request {
    req: RequestBuilder,
    method: RequestType,
}

impl Request {
    pub fn get(url: &str) -> Self {
        Request {
            req: Client::new().get(url),
            method: RequestType::Get,
        }
    }

    pub fn post(url: &str) -> Self {
        Request {
            req: Client::new().post(url),
            method: RequestType::Post,
        }
    }

    pub fn data(mut self, data: &Value) -> Self {
        self.req = match self.method {
            RequestType::Get => self.req.query(&data),
            RequestType::Post => self.req.json(&data),
        };
        self
    }

    pub async fn sign(mut self, manager: &mut TokenManager) -> Result<Self, WechatError> {
        let token = manager.get_access_token().await?.string();
        let params = json!({ "access_token": token });
        self.req = self.req.query(&params);
        Ok(self)
    }

    pub async fn send(self) -> Result<Value, WechatError> {
        use super::errors::GetKey;
        let data = self.req.send().await?.json::<Value>().await?;
        if let Some(_) = data.get("errcode") {
            let errcode = data.get_key("errcode")?;
            let errmsg = data.get_key("errmsg")?;
            if errcode != 0 {
                log::warn!("wechat error: {}", data);
                return Err(WechatError::Wechat { errcode, errmsg });
            }
        }
        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;
    #[tokio::test]
    async fn test_send() {
        let data = json!({});
        let mut manager = TokenManager::new("test".to_owned(), "test".to_owned());
        let request = Request::post("https://api.weixin.qq.com/cgi-bin/message/template/send")
            .data(&data)
            .sign(&mut manager)
            .await;
        // this should fail since there's no valid token manager
        assert!(request.is_err());
    }
}
