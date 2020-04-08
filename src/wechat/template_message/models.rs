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

    // below are generated fields, do not expect from user input.
    /// id for the message, will be generated at handler function
    pub id: Option<uuid::Uuid>,
    // the detail url pushed to wechat which receiver will open to see the detailed message
    pub detail_url: Option<String>,
}
