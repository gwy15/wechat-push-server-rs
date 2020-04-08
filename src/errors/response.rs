use super::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    errmsg: String,
    detail: Option<String>,
}

impl From<&Error> for ErrorResponse {
    fn from(e: &Error) -> Self {
        use Error::*;
        let errmsg = match e {
            InternalError(_) | OtherInternal(_) => "Internal Error".to_owned(),
            Unauthorized(s) | BadRequest(s) | NotFound(s) => s.clone(),
        };
        let detail = match e {
            InternalError(e) => Some(format!("{}", e)),
            OtherInternal(e) => Some(format!("{}", e)),
            Unauthorized(_) | BadRequest(_) | NotFound(_) => None,
        };
        Self { errmsg, detail }
    }
}
