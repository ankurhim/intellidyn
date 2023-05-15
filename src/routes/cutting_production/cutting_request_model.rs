use serde::{Serialize, Deserialize };
use chrono::{ DateTime, Local, NaiveDate};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteelRequisition {
    pub request_pk: Uuid,
    pub planned_date: NaiveDate,
    pub machine_no: String,
    pub part_no: String,
    pub heat_no: String,
    pub planned_quantity: i64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>,
    pub remarks: Option<String>
}