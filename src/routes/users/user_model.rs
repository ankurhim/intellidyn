use serde::{Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

impl User {
    pub fn default() -> Self {
        User {
            username: "Administrator".to_string(),
            password: "admin@123".to_string()
        }
    }
}