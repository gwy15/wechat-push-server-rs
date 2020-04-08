use actix_web::ResponseError;
use diesel::result::Error as DieselError;
use failure::Context;
use redis::RedisError;

use crate::wechat::errors::WechatError;

#[derive(Debug, Fail)]
pub enum InternalError {
    // database error
    #[fail(display = "Error executing DB operation")]
    Database(#[fail(cause)] DieselError),

    // redis error
    #[fail(display = "Error executing redis operation.")]
    Redis(#[fail(cause)] RedisError),

    // wechat
    #[fail(display = "Error calling wechat api")]
    Wechat(#[fail(cause)] WechatError),
}

/// Error type for all Result for app handlers.
/// 
/// For non-user-facing error (internal errors), DieselError, RedisError and WechatError
/// can be converted into Error::InternalError directly, with a cause to trace back. 
/// For other internal errors, use Result<>.context(s) to convert to Error::OtherInternal.
/// 
/// For user-facing errors, Use Error::Unauthorized(String) and etc. to generate a json response
/// like { errmsg: string }.
/// 
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Internal error happened")]
    InternalError(#[fail(cause)] InternalError),

    #[fail(display = "Internal error: {}", _0)]
    OtherInternal(Context<String>),

    // user-faced
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(String),

    #[fail(display = "Bad request: {}", _0)]
    BadRequest(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {}

// from internal errors
impl From<DieselError> for Error {
    fn from(e: DieselError) -> Self {
        Error::InternalError(InternalError::Database(e))
    }
}
impl From<RedisError> for Error {
    fn from(e: RedisError) -> Self {
        Error::InternalError(InternalError::Redis(e))
    }
}
impl From<WechatError> for Error {
    fn from(e: WechatError) -> Self {
        Error::InternalError(InternalError::Wechat(e))
    }
}
// from context
impl<T> From<Context<T>> for Error
where
    T: Into<String> + Sync + Send + std::fmt::Display,
{
    fn from(ctx: Context<T>) -> Self {
        Error::OtherInternal(ctx.map(|s| s.into()))
    }
}
