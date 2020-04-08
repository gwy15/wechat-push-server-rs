use super::super::{errors::WechatError, Request, TokenManager};
use super::NewMessage;
use serde_json::{json, Value};
use std::sync::Mutex;

pub async fn send_template_message(
    token_manager: &Mutex<TokenManager>,
    message: NewMessage,
) -> Result<Value, WechatError> {
    const URL: &'static str = "https://api.weixin.qq.com/cgi-bin/message/template/send";
    let mut data = json!({
        "touser": message.receiver,
        "template_id": message.template_id.expect("Template ID should not be none"),
        "data": {
            "title": message.title,
            "body": message.body
        }
    });
    if let Some(url) = message.url {
        data["data"]["url"] = json!(url);
    }

    let mut request = Request::post(URL).data(&data);
    {
        let mut guard = token_manager.lock().unwrap();
        request = request.sign(&mut guard).await?;
    }
    let response = request.send().await?;
    Ok(response)
}
