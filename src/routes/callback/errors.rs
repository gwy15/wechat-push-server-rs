use actix_web::ResponseError;

#[derive(Debug, Fail)]
pub enum CallbackError {
    #[fail(display = "Failed to parse callback xml")]
    Xml,
}

impl ResponseError for CallbackError {}
