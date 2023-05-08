use uuid::Uuid;
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth{
    pub auth_pk: Uuid,
    pub username: String,
    pub auth: String,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}