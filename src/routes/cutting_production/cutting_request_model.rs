use serde::{Serialize, Deserialize };

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteelRequisition {
    pub request_pk: Uuid,
    pub part_no: String,
    pub cutting_quantity: i64,
    pub created_by: Option<String>,
    pub created_on: std::time::SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<std::time::SystemTime>
}