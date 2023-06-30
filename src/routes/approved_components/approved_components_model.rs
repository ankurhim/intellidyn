use serde::{Serialize, Deserialize };
use chrono::{Local, DateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedComponent {
    pub approval_pk: Uuid,
    pub rm_id: String,
    pub heat_no: String,
    pub approved_part: String,
    pub avail_qty: f64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}