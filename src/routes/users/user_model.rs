use serde::{Serialize, Deserialize };
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_pk: Uuid,
    pub username: String,
    pub password: String,
    pub created_by: Option<String>,
    pub created_on: SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<SystemTime>
}

impl User {
    pub fn default() -> Self {
        User {
            user_pk: Uuid::new_v4(),
            username: "Administrator".to_string(),
            password: "admin@123".to_string(),
            created_by: None,
            created_on: SystemTime::now(),
            modified_by: None,
            modified_on: None
        }
    }
}