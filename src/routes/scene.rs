use actix_web::{web, HttpResponse};
use redis::AsyncCommands;
use serde_json::json;

use crate::errors::Result;
use crate::shared_state::AppState;
use crate::wechat;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/scene")
            .name("Create new scene")
            .route(web::post().to(create_scene)),
    )
    .service(
        web::resource("/scene/{scene_id}")
            .name("Query scene with id")
            .route(web::get().to(query_scene)),
    );
}

/// Creates a scan scene.
///
/// basically it calls the [wechat API]
/// [wechat API]: https://developers.weixin.qq.com/doc/offiaccount/Account_Management/Generating_a_Parametric_QR_Code.html
async fn create_scene(state: web::Data<AppState>) -> Result<HttpResponse> {
    // make a new QR Code scan scene
    let scene = wechat::qrcode::create_new_temp(&state.token_manager, 5 * 60).await?;
    log::info!("New scene generated");
    let scene_id = scene["scene_id"].as_u64().unwrap();
    let ticket = scene["ticket"].as_str().unwrap();
    let qr_url = format!(
        "https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket={}",
        ticket
    );

    let response = json!({
        "scene_id": scene_id,
        "ticket": ticket,
        "qr_url": qr_url,
    });

    Ok(HttpResponse::Ok().json(response))
}

async fn query_scene(query: web::Path<(u32,)>, state: web::Data<AppState>) -> Result<HttpResponse> {
    let scene_id = query.0;
    // query from redis
    let mut con = state.as_ref().redis_connection().await?;
    let key: String = format!("scene_{}", scene_id);
    let response: Option<String> = con.get(&key).await?;

    Ok(match response {
        Some(open_id) => HttpResponse::Ok().json(json!({ "openID": open_id })),
        None => HttpResponse::NotFound().json(json!({})),
    })
}
