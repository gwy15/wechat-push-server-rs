use actix_web::{web, HttpResponse, Result};

async fn message_detail(params: web::Path<(String,)>) -> Result<HttpResponse> {
    let token = &params.0;
    // TODO:
    let response = serde_json::json!({ "token": token.to_owned() });
    Ok(HttpResponse::Ok().json(response))
}

async fn post_message() -> Result<HttpResponse> {
    // TODO:
    Ok(HttpResponse::Ok().body(""))
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
