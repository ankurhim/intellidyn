use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};

use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::purchase_order::purchase_order_model::PurchaseOrder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindPurchaseOrderRequest {
    pub filter: Option<String>
}

impl FindPurchaseOrderRequest {
    pub async fn find_po_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_purchase_order_table';",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn find_all_purchase_orders(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut po_vector: Vec<PurchaseOrder> = Vec::new();
        
        let resp = service.client
        .query("SELECT * FROM mwspl_purchase_order_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            po_vector.push(PurchaseOrder {
                purchase_order_pk: Uuid::parse_str(row.get(1)).unwrap(),
                purchase_order_no: row.get(2),
                po_date: row.get(3),
                po_quantity: row.get(4),
                po_received_date: row.get(5),
                po_effective_date: row.get(6),
                po_status: row.get(7),
                po_deactive_date: row.get(8),
                rate: row.get(9),
                drawing_no: row.get(10),
                part_name: row.get(11),
                part_no: row.get(12),
                grade: row.get(13),
                section: row.get(14),
                section_type: row.get(15),
                jominy_range: row.get(16),
                gross_weight: row.get(17),
                cut_weight: row.get(18),
                created_by: row.get(19),
                created_on: row.get(20),
                created_login_key: row.get(21),
                modified_by: row.get(22),
                modified_on: row.get(23),
                modified_login_key: row.get(24),
                remarks: row.get(25)
            })
        };
        match &po_vector.len() {
            0 => Json(json!(None::<Vec<PurchaseOrder>>)),
            _ => Json(json!(po_vector))
        }
    }

    pub async fn find_all_purchase_orders_by_po_no(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(payload): Query<FindPurchaseOrderRequest>
    ) -> Json<Value> {

        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut po_vector: Vec<PurchaseOrder> = Vec::new();
        
        let resp = service.client
        .query("SELECT * FROM mwspl_purchase_order_table WHERE purchase_order_no = $1;", &[&payload.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            po_vector.push(PurchaseOrder {
                purchase_order_pk: Uuid::parse_str(row.get(1)).unwrap(),
                purchase_order_no: row.get(2),
                po_date: row.get(3),
                po_quantity: row.get(4),
                po_received_date: row.get(5),
                po_effective_date: row.get(6),
                po_status: row.get(7),
                po_deactive_date: row.get(8),
                rate: row.get(9),
                drawing_no: row.get(10),
                part_name: row.get(11),
                part_no: row.get(12),
                grade: row.get(13),
                section: row.get(14),
                section_type: row.get(15),
                jominy_range: row.get(16),
                gross_weight: row.get(17),
                cut_weight: row.get(18),
                created_by: row.get(19),
                created_on: row.get(20),
                created_login_key: row.get(21),
                modified_by: row.get(22),
                modified_on: row.get(23),
                modified_login_key: row.get(24),
                remarks: row.get(25)
            })
        };
        match &po_vector.len() {
            0 => Json(json!(None::<Vec<PurchaseOrder>>)),
            _ => Json(json!(po_vector))
        }
    }
}