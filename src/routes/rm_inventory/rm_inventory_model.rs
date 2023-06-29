use serde::{Serialize, Deserialize };
use chrono::{Local, DateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RMInventory {
    pub heat_no: String,
    pub grade: String,
    pub size: usize,
    pub section: String,
    pub avail_qty: f64,
    pub rm_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}