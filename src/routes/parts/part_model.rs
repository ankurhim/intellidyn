use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part{
    pub part_pk: Uuid,
    pub part_code: String,
    pub part_no: String,
    pub part_name: String,
    pub dwg_rev_no: String,
    pub steel_code: String,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub part_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}