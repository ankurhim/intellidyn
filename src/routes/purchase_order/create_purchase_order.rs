use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{
    Extension,
    Json,
    extract::{Path, Query}
};

use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;
use crate::routes::log::find_logs::FindLogRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePurchaseOrderRequest {
    pub purchase_order_no: String,
    pub po_date: String,
    pub po_quantity: Option<i64>,
    pub po_received_date: Option<String>,
    pub po_effective_date: Option<String>,
    pub po_status: String,
    pub po_deactive_date: Option<String>,
    pub rate: f64,
    pub remarks: Option<String>
}

impl CreatePurchaseOrderRequest {
    pub async fn create_purchase_order_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_purchase_order_table(
                id SERIAL,
                purchase_order_pk TEXT NOT NULL,
                purchase_order_no TEXT NOT NULL PRIMARY KEY,
                po_date DATE NOT NULL,
                po_quantity BIGINT,
                po_received_date DATE,
                po_effective_date DATE,
                po_status TEXT NOT NULL,
                po_deactive_date DATE,
                rate FLOAT8 NOT NULL,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON DELETE CASCADE,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON DELETE CASCADE,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (purchase_order_no)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_purchase_order_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let drop_purchase_order_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_purchase_order_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match drop_purchase_order_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_purchase_order(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {

        let log_key = FindLogRequest::find_active_log_by_username(Extension(service.clone()), Query(FindLogRequest { username: Some(user.clone()) })).await;

        let po_date = NaiveDate::parse_from_str(&payload.po_date, "%d-%m-%Y").expect("PO Date parsing error");
        let po_received_date = match &payload.po_received_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Received Date parsing error")),
            None => None
        };
        let po_effective_date =  match &payload.po_effective_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Effective Date parsing error")),
            None => None
        };
        let po_deactive_date = match &payload.po_deactive_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Deactive Date parsing error")),
            None => None
        };
        
        match service.client
        .execute(
            "INSERT INTO mwspl_purchase_order_table(
                purchase_order_pk,
                purchase_order_no,
                po_date,
                po_quantity,
                po_received_date,
                po_effective_date,
                po_status,
                po_deactive_date,
                rate,
                created_by,
                created_on,
                modified_by,
                modified_on,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.purchase_order_no,
                &po_date,
                &payload.po_quantity,
                &po_received_date,
                &po_effective_date,
                &payload.po_status,
                &po_deactive_date,
                &payload.rate,
                &user,
                &Local::now(),
                &None::<String>,
                &None::<DateTime<Local>>,
                &payload.remarks
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