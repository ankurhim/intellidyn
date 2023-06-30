use serde::{Serialize, Deserialize };
use chrono::{Local, DateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RMInventory {
    pub heat_no: String,
    pub grade: String,
    pub size: i64,
    pub section: String,
    pub avail_qty: f64
}