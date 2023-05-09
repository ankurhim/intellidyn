use serde::{Serialize, Deserialize };
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Query, Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePurchaseOrderRequest {
    pub purchase_order_no: String,
    pub drawing_no: String,
    pub po_status: String,
}

impl UpdatePurchaseOrderRequest {
    pub async fn update_po_status_by_filter(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(payload): Query<UpdatePurchaseOrderRequest>
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
        
        match service.client
        .execute(
            "UPDATE
                mwspl_purchase_order_table
            SET
                po_status = $3,
                modified_by = $4,
                modified_on = $5,
                modified_login_key = $6,
                po_deactive_date = $7
            WHERE
                purchase_order_no = $1 AND
                drawing_no = $2;",
                &[
                    &payload.purchase_order_no,
                    &payload.drawing_no,
                    &payload.po_status,
                    &user,
                    &Local::now(),
                    &login_key,
                    &Local::now().date_naive()
                ]
            )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}