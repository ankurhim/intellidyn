use serde::{Serialize, Deserialize };
use time::Date;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingSteel {
    pub incoming_pk: Uuid,
    pub challan_no: String,
    pub challan_date: Date,
    pub grade: String,
    pub section: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub jominy_value: Option<String>,
    pub received_qty: i64,
    pub created_by: Option<String>,
    pub created_on: std::time::SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<std::time::SystemTime>
}

impl IncomingSteel {
    pub fn default() -> Self {
        IncomingSteel {
            incoming_pk: Uuid::new_v4(),
            challan_no: "".to_string(),
            challan_date: Date::MIN,
            grade: "".to_string(),
            section: "".to_string(),
            heat_no: "".to_string(),
            heat_code: None,
            jominy_value: None,
            received_qty: 0,
            created_by: None,
            created_on: std::time::SystemTime::now(),
            modified_by: None,
            modified_on: None
        }
    }
}