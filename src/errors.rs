use actix_web::ResponseError;
use redis::RedisError;

use crate::wechat::errors::WechatError;

#[derive(Debug, Fail)]
pub enum Error {
    // database error
    #[fail(display = "Error executing DB operation")]
    Database,

    // redis error
    #[fail(display = "Error executing redis operation.")]
    Redis(#[fail(cause)] RedisError),

    // wechat
    #[fail(display = "Error calling wechat api")]
    Wechat(#[fail(cause)] WechatError),
}

impl ResponseError for Error {}

impl From<RedisError> for Error {
    fn from(e: RedisError) -> Self {
        Error::Redis(e)
    }
}

impl From<WechatError> for Error {
    fn from(e: WechatError) -> Self {
        Error::Wechat(e)
    }
}
