use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::{ DateTime, Local, Month };

#[derive(Debug, Clone Serialize, Deserialize)]
pub struct Schedule {
    pub schedule_pk: Uuid,
    pub schedule_month: Month,
    pub schedule_year: i64,
    pub drawing_no: String,
    pub similar_part_no: Option<String>,
    pub customer_plant: String,
    pub supplier_plant: String,
    pub most_critical_qty: Option<i64>,
    pub most_critical_commitment_date: Option<NaiveDate>,
    pub critical_qty: Option<i64>,
    pub critical_commitment_date: Option<NaiveDate>,
    pub normal_qty: Option<i64>,
    pub normal_commitment_date: Option<NaiveDate>,
    pub total_forging_qty: i64,
    pub recv_till: Option<i64>,
    pub balance_qty: i64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>,
    pub remarks: Option<String>
}