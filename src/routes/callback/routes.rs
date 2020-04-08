use crate::errors::{Error, Result};
use crate::shared_state::AppState;
use actix_web::{web, HttpResponse};
use failure::ResultExt;
use redis::aio::Connection as RedisConnection;
use redis::AsyncCommands;

use super::verification::WechatQuery;

/// insert scene_id -> open_id into cache.
async fn cache_scene_id_with_openid(
    mut redis_connection: RedisConnection,
    scene: String,
    open_id: String,
) -> Result<()> {
    // insert
    redis_connection
        .set_ex(format!("scene_{}", scene), open_id, 5 * 60)
        .await?;
    Ok(())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/callback")
            .route(web::get().to(echo_get_callback))
            .route(web::post().to(event_callback)),
    );
}

/// GET /callback, return echostr
async fn echo_get_callback(
    query: web::Query<WechatQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    if !query.verify(state.as_ref().config.wechat.token.as_str()) {
        return Err(Error::Unauthorized(
            "Callback signature verification failed".to_owned(),
        ));
    }
    let echostr = query
        .echostr
        .as_ref()
        .ok_or(Error::BadRequest("missing query echostr".to_owned()))?;
    Ok(HttpResponse::Ok().body(echostr))
}

/// POST /callback, event callback
async fn event_callback(
    query: web::Query<WechatQuery>,
    state: web::Data<AppState>,
    body: String,
) -> Result<HttpResponse> {
    if !query.verify(state.as_ref().config.wechat.token.as_str()) {
        return Err(Error::Unauthorized("Verification failed".to_owned()));
    }
    use super::xml_parse::parse_xml_string;
    // parse body
    let data = parse_xml_string(body).context("Paring callback xml error")?;

    // |_| Error::internal("Paring callback xml error"))?;
    match data["MsgType"].as_str() {
        "event" => on_event(state, data).await,
        t => {
            log::debug!("Unknown Type {}", t);
            Ok(HttpResponse::Ok().body(""))
        }
    }
}

async fn on_event(
    state: web::Data<AppState>,
    data: std::collections::HashMap<String, String>,
) -> Result<HttpResponse> {
    let open_id = data["FromUserName"].clone();
    match data["Event"].as_str() {
        // on subscribe or scan
        "subscribe" | "scan" => {
            match data.get("EventKey") {
                None => {}
                Some(value) => {
                    // save event key
                    let scene_id = match value.starts_with("qrscene_") {
                        true => value.replace("qrscene_", ""),
                        false => value.clone(),
                    };
                    // insert into database
                    cache_scene_id_with_openid(
                        state.as_ref().redis_connection().await?,
                        scene_id,
                        open_id,
                    )
                    .await?;
                }
            }
        }
        event => log::debug!("Unknown event {}", event),
    }
    Ok(HttpResponse::Ok().body(""))
}
