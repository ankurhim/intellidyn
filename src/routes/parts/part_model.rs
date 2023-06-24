use serde::{Serialize, Deserialize };
use uuid::Uuid;
use crate::steels::steel_model::Steel;

use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part{
    pub part_pk: Uuid,
    pub part_code: String,
    pub part_no: String,
    pub part_name: String,
    pub dwg_rev_no: String,
    pub steel_grade: Steel,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub cut_length: Option<f64>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>,
    pub remarks: Option<String>

}