use serde::{Serialize, Deserialize};
use chrono::{ naive::NaiveDate, DateTime, Local };
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuttingMaterial {
    pub cutting_material_pk: Uuid,
    pub cutting_date: NaiveDate,
    pub drawing_no: String,
    pub available_qty: i64,
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub quality_status: String,
    pub batch_status: String,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>,
    pub remarks: Option<String>
}