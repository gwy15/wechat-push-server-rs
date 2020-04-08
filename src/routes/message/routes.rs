use crate::errors::Result;
use crate::shared_state::AppState;
use crate::wechat::template_message::{apis, NewMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

async fn message_detail(params: web::Path<(String,)>) -> Result<HttpResponse> {
    let token = &params.0;
    // TODO:
    let response = serde_json::json!({ "token": token.to_owned() });
    Ok(HttpResponse::Ok().json(response))
}

async fn post_message(
    message: web::Form<NewMessage>,
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    // extract from web::Form boxing
    let mut message: NewMessage = message.into_inner();
    // first merge with default template_id
    message.template_id = Some(
        message
            .template_id
            .unwrap_or_else(|| state.as_ref().config.wechat.default_template_id.clone()),
    );
    // post message with wechat module api
    apis::send_template_message(&state.as_ref().token_manager, message.clone()).await?;
    // if success, write to database
    use crate::models::Message;
    use std::time::SystemTime;
    use uuid::Uuid;
    // init msg
    let msg = Message {
        id: Uuid::new_v4(),
        app_id: state.as_ref().config.wechat.app_id.clone(),
        template_id: message.template_id.unwrap(),
        receiver_id: message.receiver,
        title: message.title,
        body: message.body.unwrap_or_default(),
        url: message.url,
        created_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        ip: request
            .connection_info()
            .remote()
            .unwrap_or("Failed to parse IP address")
            .into(),
        UA: crate::utils::get_user_agent(&request)
            .map_err(|e| {
                log::warn!("Bad UA: {}", e);
                e
            })
            .unwrap_or_default(),
    };
    // insert into database

    Ok(HttpResponse::Ok().json(json!({})))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/message/{token}")
            .name("message_detail")
            .route(web::get().to(message_detail)),
    )
    .service(
        web::resource("/message")
            .name("post new message")
            .route(web::post().to(post_message)),
    );
}
