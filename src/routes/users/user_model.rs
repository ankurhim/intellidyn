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
    pub phone_no: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}

impl User {
    pub fn default() -> Self {
        User {
            user_pk: Uuid::new_v4(),
            full_name: "Administrator".to_string(),
            employee_id: "".to_string(),
            username: "admin".to_string(),
            password: None,
            phone_no: None,
            created_by: None,
            created_on: Local::now(),
            modified_by: None,
            modified_on: None,
            remarks: None
        }
    }
}