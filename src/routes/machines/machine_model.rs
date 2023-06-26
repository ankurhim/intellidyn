use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub machine_pk: Uuid,
    pub machine_id: String,
    pub machine_type: String,
    pub machine_name: String,
    pub machine_location: String,
    pub machine_model: String,
    pub machine_capacity: Option<String>,
    pub machine_status: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}