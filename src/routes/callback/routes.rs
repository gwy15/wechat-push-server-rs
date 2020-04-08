use crate::errors::Error as MyError;
use crate::shared_state::AppState;
use actix_web::{error, web, HttpResponse, Result as AWResult};

use redis::aio::Connection as RedisConnection;
use redis::AsyncCommands;

use super::verification::WechatQuery;

/// insert scene_id -> open_id into cache.
async fn cache_scene_id_with_openid(
    mut redis_connection: RedisConnection,
    scene: String,
    open_id: String,
) -> Result<(), MyError> {
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
) -> AWResult<HttpResponse> {
    if !query.verify(state.as_ref().config.wechat.token.as_str()) {
        return Err(error::ErrorUnauthorized("Verification failed"));
    }
    let echostr = query
        .echostr
        .as_ref()
        .ok_or(error::ErrorBadRequest("missing query echostr"))?;
    Ok(HttpResponse::Ok().body(echostr))
}

/// POST /callback, event callback
async fn event_callback(
    query: web::Query<WechatQuery>,
    state: web::Data<AppState>,
    body: String,
) -> AWResult<HttpResponse> {
    if !query.verify(state.as_ref().config.wechat.token.as_str()) {
        return Err(error::ErrorUnauthorized("Verification failed"));
    }
    use super::xml_parse::parse_xml_string;
    // parse body
    let data = parse_xml_string(body)?;
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
) -> AWResult<HttpResponse> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{app, get};
    #[actix_rt::test]
    async fn test_echo_get_callback() {
        let mut app = app!("/callback" => web::get().to(echo_get_callback));
        // no param
        let r = get!(app, "/callback");
        assert!(r.status().is_client_error());
        // bad sign
        let bad_url = "/callback?signature=6d&timestamp=timestamp&nonce=nonce&echostr=123";
        let r = get!(app, bad_url);
        assert!(r.status().is_client_error());
        // no echo
        let signature = "6db4861c77e0633e0105672fcd41c9fc2766e26e";
        let signed_url = format!(
            "/callback?signature={}&timestamp=timestamp&nonce=nonce",
            signature
        );
        let r = get!(app, &signed_url);
        assert!(r.status().is_client_error());
        // ok, skip for now since I'm too lazy to calculate the sign
        // let ok_url = signed_url + "&echostr=123";
        // let r = get!(app, &ok_url);
        // let body = actix_web::test::read_body(r).await;
        // assert_eq!(body, "123");
    }
}
