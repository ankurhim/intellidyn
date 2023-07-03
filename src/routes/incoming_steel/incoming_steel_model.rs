use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, naive::NaiveDate, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingSteel {
    pub incoming_steel_pk: Uuid,
    pub challan_no: String,
    pub challan_date: NaiveDate,
    pub steel_code: String,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub jominy_value: Option<String>,
    pub received_qty: f64,
    pub avail_qty: f64,
    pub heat_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}