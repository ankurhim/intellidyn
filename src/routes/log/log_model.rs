use uuid::Uuid;
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub log_pk: Uuid,
    pub username: String,
    pub login_key: String,
    pub login_time: DateTime<Local>,
    pub logout_time: Option<DateTime<Local>>,
    pub remarks: Option<String>
}