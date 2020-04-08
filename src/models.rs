use serde::{Deserialize, Serialize};

use crate::schema::messages;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable)]
#[allow(non_snake_case)]
pub struct Message {
    pub id: Uuid,
    pub app_id: String,
    pub template_id: String,
    pub receiver_id: String,
    pub title: String,
    pub body: String,
    pub url: Option<String>,

    pub created_time: i64,
    pub ip: String,
    pub UA: String,
    pub errcode: Option<i32>,
    pub msgid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub template_id: Option<String>,
    pub receiver: String,
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>,
}
