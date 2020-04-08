#[macro_use]
extern crate failure;

#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::web;

mod config;
mod errors;
mod logging;
mod models;
mod routes;
mod schema;
mod shared_state;
mod wechat;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    logging::init_logger();

    // init wechat token manager here
    let config = config::Config::new(false).unwrap();
    let root_url = config.root_url.clone();
    let state = shared_state::AppState::from_config(config);
    // pre-wrap with Arc to avoid clone state
    let app_data = web::Data::new(state);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(
                web::scope(&root_url)
                    .configure(routes::scene::configure)
                    .configure(routes::message::configure)
                    .configure(routes::callback::configure),
            )
            .default_service(web::route().to(routes::default_handler))
    })
    .bind("127.0.0.1:8088")?;

    let server_future = server.run();
    server_future.await
}
