use crate::errors::Result;
use actix_web::HttpRequest;
use failure::ResultExt;

pub fn get_user_agent(request: &HttpRequest) -> Result<String> {
    let header_value = request.headers().get(actix_web::http::header::USER_AGENT);
    match header_value {
        None => Ok("".into()),
        Some(header_value) => Ok(header_value
            .to_str()
            .context("Non-visible ASCII chars in header ua.")?
            .to_owned()),
    }
}
