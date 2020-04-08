use super::super::{errors::WechatError, Request, TokenManager};
use super::NewMessage;
use serde_json::{json, Value};
use std::sync::Mutex;

pub async fn send_template_message(
    token_manager: &Mutex<TokenManager>,
    message: &NewMessage,
) -> Result<Value, WechatError> {
    const URL: &'static str = "https://api.weixin.qq.com/cgi-bin/message/template/send";
    let data = json!({
        "touser": message.receiver,
        "template_id": message.template_id.as_ref().expect("Template ID should not be none"),
        "url": message.detail_url,
        "data": {
            "title": {
                "value": message.title
            },
            "body": {
                "value": message.body.as_ref().unwrap_or(&"".to_owned())
            }
        },
    });
    log::trace!("Sending data to wechat: {}", data);

    let mut request = Request::post(URL).data(&data);
    {
        let mut guard = token_manager.lock().unwrap();
        request = request.sign(&mut guard).await?;
    }
    let response = request.send().await?;
    Ok(response)
}
