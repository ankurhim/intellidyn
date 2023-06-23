use uuid::Uuid;
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_pk: Uuid,
    pub full_name: String,
    pub employee_id: String,
    pub username: String,
    pub password: Option<String>,
    pub email_id: Option<String>,
    pub role: String,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}