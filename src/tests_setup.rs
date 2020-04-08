#![cfg(test)]

#[macro_export]
macro_rules! app {
    ($path:expr => $fn:expr) => {{
        use crate::shared_state::AppState;
        use actix_web::{test, App};
        let config = crate::config::Config::load(true).unwrap();
        let state = web::Data::new(AppState::new(config));
        test::init_service(App::new().app_data(state).route($path, $fn)).await
    }};
}

#[macro_export]
macro_rules! get {
    ($app:expr, $uri:expr) => {{
        use actix_web::test;
        let request = test::TestRequest::get().uri($uri).to_request();
        actix_web::test::call_service(&mut $app, request).await
    }};
}
