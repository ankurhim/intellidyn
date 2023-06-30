use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Steel{
    pub steel_pk: Uuid,
    pub steel_code: String,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub steel_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>

}