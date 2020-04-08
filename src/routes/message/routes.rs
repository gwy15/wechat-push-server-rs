use crate::errors::Result;
use crate::models::Message;
use crate::shared_state::AppState;
use crate::wechat::template_message::{apis, NewMessage};
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use uuid::Uuid;

async fn message_detail(
    params: web::Path<(Uuid,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let uuid = params.0;
    // TODO: load cache from redis


    let message = web::block(move || {
        let con = state.as_ref().db_pool.get()?;
        super::actions::find_message_by_uuid(uuid, &con)
    })
    .await?;
    // cache result to redis

    if let None = message {
        return Ok(HttpResponse::NotFound().json(json!({})));
    }
    let message = message.unwrap();
    Ok(HttpResponse::Ok().json(json!({
        "title": message.title,
        "body": message.body,
        "url": message.url,
        "created_time": message.created_time
    })))
}

async fn post_message(
    message: web::Form<NewMessage>,
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    // extract from web::Form boxing
    let mut message: NewMessage = message.into_inner();
    // modify the message
    message.id = Some(Uuid::new_v4());
    message.detail_url = Some(format!(
        "{}/{}",
        state.as_ref().config.wechat.detail_url,
        message.id.as_ref().unwrap()
    ));
    message.template_id = Some(
        message
            .template_id
            .unwrap_or_else(|| state.as_ref().config.wechat.default_template_id.clone()),
    );
    log::trace!("Sending message {:?}", message);
    // post message with wechat module api
    let response = apis::send_template_message(&state.as_ref().token_manager, &message).await?;
    log::info!("A template message was sent successfully");
    // if success, write to database
    use std::time::SystemTime;
    // init msg
    let msg = Message {
        id: message.id.unwrap(),
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
    // log::debug!("Inserting {:?} into database", msg);
    web::block(move || {
        let con = state.as_ref().db_pool.get()?;
        super::actions::insert_message(msg, &con)
    })
    .await?;

    // TODO: save to redis

    Ok(HttpResponse::Ok().json(response))
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
