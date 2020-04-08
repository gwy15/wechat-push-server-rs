pub mod callback;
pub mod message;
pub mod scene;

use crate::errors::{Error, Result};
use actix_web::HttpResponse;
pub async fn default_handler() -> Result<HttpResponse> {
    Err(Error::NotFound(
        "The requested path was not found.".to_owned(),
    ))
}
