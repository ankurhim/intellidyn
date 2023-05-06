use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfMaterial {
    pub bom_pk: Uuid,
    pub purchase_order_no: String,
    pub po_quantity: Option<i64>,
    pub po_received_date: NaiveDate,
    pub po_effective_date: NaiveDate,
    pub po_status: String,
    pub po_deactive_date: NaiveDate,
    pub po_date: NaiveDate,
    pub drawing_no: String,
    pub part_name: String,
    pub part_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub rate: f64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}