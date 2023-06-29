
use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cutting {
    pub cutting_pk: Uuid,
    pub requisition_id: String,
    pub rm_id: String,
    pub planned_date: NaiveDate,
    pub machine: String,
    pub part_no: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub planned_qty: i64, 
    pub actual_qty: Option<i64>,
    pub ok_qty: Option<i64>,
    pub rej_qty: Option<i64>,
    pub ok_wt: Option<f64>,
    pub rej_wt: Option<f64>,
    pub end_pc_wt: Option<f64>,
    pub total_wt: Option<f64>,
    pub issued_qty: Option<i64>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>
}