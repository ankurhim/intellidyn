use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path, Query}, http::response::Response, http::StatusCode };
use serde_json::{Value, json};
use tokio_postgres::Row;
use crate::service::DbService;

use crate::routes::cutting_store::cutting_store_model::CuttingMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCuttingInventoryRequest {
    pub drawing_no: Option<String>,
}

impl FindCuttingInventoryRequest {
    pub async fn find_cutting_inventory(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindCuttingInventoryRequest>
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

        let service_resp = service.client
        .query(
            "SELECT * FROM mwspl_cutting_inventory_table WHERE batch_status = 'NOT ISSUED';",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(service_resp.unwrap())
    }

    pub async fn find_cutting_inventory_by_dwg_no(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindCuttingInventoryRequest>,
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

        let service_resp = service.client
        .query(
            "SELECT * FROM mwspl_cutting_inventory_table WHERE drawing_no = $1 AND batch_status = 'NOT ISSUED';",
            &[&query.drawing_no]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(service_resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<CuttingMaterial> = Vec::new();
    
    for row in row_vector {
        vector.push(CuttingMaterial {
            cutting_store_pk: Uuid::parse_str(row.get(1)).unwrap(),
            cutting_date: row.get(2),
            drawing_no: row.get(3),
            available_qty: row.get(4),
            heat_no: row.get(5),
            grade: row.get(6),
            section: row.get(7),
            section_type: row.get(8),
            gross_weight: row.get(9),
            cut_weight: row.get(10),
            quality_status: row.get(11),
            batch_status: row.get(12),
            created_by: row.get(13),
            created_on: row.get(14),
            created_login_key: row.get(15),
            modified_by: row.get(16),
            modified_on: row.get(17),
            modified_login_key: row.get(18),
            remarks: row.get(19)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<CuttingMaterial>>)),
        _ => Json(json!(vector))
    }
}