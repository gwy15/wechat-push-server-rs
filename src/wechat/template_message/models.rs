use serde::{Deserialize, Serialize};

/// The form parsed directly from web request
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub receiver: String,
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>,
    // template_id should be replace with default template id before
    // passing to wechat module
    pub template_id: Option<String>,
}
