use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfMaterial {
    pub bom_pk: Uuid,
    pub part_no: String,
    pub part_name: String,
    pub part_code: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}