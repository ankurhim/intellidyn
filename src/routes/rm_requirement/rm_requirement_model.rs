use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::{ DateTime, Local, Month, NaiveDate };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RMRequirement {
    pub rm_requirement_pk: Uuid,
    pub schedule_id: String,
    pub part_code: String,
    pub required_qty: f64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}