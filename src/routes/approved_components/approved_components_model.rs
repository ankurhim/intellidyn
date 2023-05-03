use serde::{Serialize, Deserialize };
use time::Date;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedComponent {
    pub approval_pk: Uuid,
    pub heat_no: String,
    pub section: i64,
    pub section_type: String,
    pub approved_part: String,
    pub created_by: Option<String>,
    pub created_on: std::time::SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<std::time::SystemTime>
}

impl ApprovedComponent {
    pub fn default() -> Self {
        ApprovedComponent {
            approval_pk: Uuid::new_v4(),
            heat_no: "".to_string(),
            section: 0,
            section_type: "".to_string(),
            approved_part: "".to_string(),
            created_by: None,
            created_on: std::time::SystemTime::now(),
            modified_by: None,
            modified_on: None
        }
    }
}