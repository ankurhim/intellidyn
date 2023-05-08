use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub purchase_order_pk: Uuid,
    pub purchase_order_no: String,
    pub po_date: NaiveDate,
    pub po_quantity: Option<i64>,
    pub po_received_date: Option<NaiveDate>,
    pub po_effective_date: Option<NaiveDate>,
    pub po_status: String,
    pub po_deactive_date: Option<NaiveDate>,
    pub rate: f64,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub remarks: Option<String>
}