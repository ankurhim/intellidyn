use serde::{Serialize, Deserialize };

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedComponent {
    pub approval_pk: Uuid,
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub approved_part: String,
    pub created_by: Option<String>,
    pub created_on: std::time::SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<std::time::SystemTime>
}