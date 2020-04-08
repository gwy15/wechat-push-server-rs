use reqwest;
use serde_json::Value;

#[derive(Debug, Fail)]
pub enum WechatError {
    // network failed
    #[fail(display = "Wechat request network failed: {}", cause)]
    Network {
        #[fail(cause)]
        cause: reqwest::Error,
    },
    // wechat api failed
    #[fail(display = "Wechat api errcode = {}, errmsg = {}", errcode, errmsg)]
    Wechat { errcode: u64, errmsg: String },

    // error parsing wechat response json data for missing key
    #[fail(
        display = "Failed to parse wechat response json, missing key '{}'",
        key
    )]
    WechatResponseJsonMissingKey { key: String },

    // wrong type
    #[fail(
        display = "Failed to parse wechat response json, wrong type for key '{}'",
        key
    )]
    WechatResponseJsonWrongType { key: String },
}

impl From<reqwest::Error> for WechatError {
    fn from(e: reqwest::Error) -> Self {
        WechatError::Network { cause: e }
    }
}

// convert Value to type Option<T>
pub trait TryFromValue {
    fn try_from_value(f: &Value) -> Option<Self>
    where
        Self: std::marker::Sized;
}
// impl
impl TryFromValue for i64 {
    fn try_from_value(v: &Value) -> Option<Self> {
        v.as_i64()
    }
}
impl TryFromValue for u64 {
    fn try_from_value(v: &Value) -> Option<Self> {
        v.as_u64()
    }
}
impl TryFromValue for String {
    fn try_from_value(v: &Value) -> Option<Self> {
        v.as_str().and_then(|s| Some(s.to_owned()))
    }
}

// value.get_key
pub trait GetKey {
    fn get_key<V>(&self, key: &str) -> Result<V, WechatError>
    where
        V: TryFromValue;
}
impl GetKey for Value {
    fn get_key<V>(&self, key: &str) -> Result<V, WechatError>
    where
        V: TryFromValue,
    {
        let option: Option<&Value> = self.get(key);
        match option {
            Some(v) => {
                let converted: Option<V> = V::try_from_value(v);
                match converted {
                    Some(v) => Ok(v),
                    // type mismatched
                    None => Err(WechatError::WechatResponseJsonWrongType {
                        key: key.to_owned(),
                    }),
                }
            }
            // no such key
            None => Err(WechatError::WechatResponseJsonMissingKey {
                key: key.to_owned(),
            }),
        }
    }
}
