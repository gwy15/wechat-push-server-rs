use serde::Deserialize;
use std::time::SystemTime;

#[derive(Deserialize)]
pub struct WechatQuery {
    signature: String,
    timestamp: String,
    nonce: String,
    pub echostr: Option<String>,
}

impl WechatQuery {
    fn verify_timestamp(&self) -> bool {
        let ts = match self.timestamp.parse::<u64>() {
            Err(e) => {
                log::warn!("failed to parse timestamp: {}", e);
                return false;
            }
            Ok(ts) => ts,
        };
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Err(e) => {
                log::error!("System time before 0! {}", e);
                return false;
            }
            Ok(duration) => duration.as_secs(),
        };
        let diff = match ts > now {
            true => ts - now,
            false => now - ts,
        };
        if diff > 10 {
            log::warn!("Timestamp diff to much, reject.");
            return false;
        }
        true
    }
    fn verify_sign(&self, token: &str) -> bool {
        use crypto::digest::Digest;
        use crypto::sha1::Sha1;
        // verify timestamp

        // verify signature
        let mut args: Vec<&str> = vec![token, &self.timestamp, &self.nonce];
        args.sort();
        let raw = args.join("");
        let mut hasher = Sha1::new();
        hasher.input_str(&raw);
        let signature = hasher.result_str();
        // compare signature
        signature == self.signature
    }
    pub fn verify(&self, token: &str) -> bool {
        self.verify_timestamp() && self.verify_sign(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign() {
        let query = WechatQuery {
            signature: "6db4861c77e0633e0105672fcd41c9fc2766e26e".to_owned(),
            timestamp: "timestamp".to_owned(),
            nonce: "nonce".to_owned(),
            echostr: None,
        };
        assert!(!query.verify_sign("bad_token"));
        assert!(query.verify_sign("token"));
    }
}
