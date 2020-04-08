use super::errors::WechatError;
use super::Request;
use super::TokenManager;

use rand;
use serde_json::{json, Value};
use std::sync::Mutex;

pub async fn create_new_temp(
    token_manager: &Mutex<TokenManager>,
    expires: i32,
) -> Result<Value, WechatError> {
    const URL: &'static str = "https://api.weixin.qq.com/cgi-bin/qrcode/create";
    // generate a random scene id for use
    let scene_id = rand::random::<u32>();

    let data = json!({
        "expire_seconds": expires,
        "action_name": "QR_SCENE",
        "action_info": {
            "scene": {
                "scene_id": scene_id
            }
        }
    });

    let mut request = Request::post(URL).data(&data);
    {
        let mut guard = token_manager.lock().unwrap();
        request = request.sign(&mut guard).await?;
    }
    let mut resp = request.send().await?;
    resp["scene_id"] = Value::Number(serde_json::Number::from(scene_id));
    Ok(resp)
}
