use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{DateTime, naive::NaiveDate, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requisition {
    pub requisition_pk: Uuid,
    pub request_from: String,
    pub request_to: String,
    pub part_no: String,
    pub requested_qty: i64,
    pub comments: Option<String>,
    pub request_status: String,
    pub reply:Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}