use serde::{Serialize, Deserialize };
use time::Date;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingSteel {
    pub incoming_pk: Uuid,
    pub challan_no: String,
    pub challan_date: Date,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub jominy_value: Option<String>,
    pub received_qty: i64,
    pub issued_qty: i64,
    pub actual_qty: i64,
    pub heat_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: std::time::SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<std::time::SystemTime>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteelInventory {
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub heat_code: String,
    pub total_received_qty: i64,
    pub total_issued_qty: i64,
    pub total_available_qty: i64,
    pub heat_status: Option<String>
}