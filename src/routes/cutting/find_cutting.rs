use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use tokio_postgres::Row;
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::cutting::cutting_model::Cutting;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCuttingRequest {
    pub filter: Option<String>
}

impl FindCuttingRequest {
    pub async fn find_all_cuttings(
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
        
        let resp = service.client
        .query("SELECT * FROM mwspl_cutting_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Cutting> = Vec::new();
    
    for row in row_vector {
        vector.push(Cutting {
            cutting_pk: row.get(1),
            requisition_id: row.get(2),
            rm_id: row.get(3),
            planned_date: row.get(4),
            machine: row.get(5),
            part_no: row.get(6),
            heat_no: row.get(7),
            heat_code: row.get(8),
            steel_grade: row.get(9),
            section: row.get(10),
            section_type: row.get(11),
            planned_qty: row.get(12),
            actual_qty: row.get(13),
            ok_qty: row.get(14),
            rej_qty: row.get(15),
            ok_wt: row.get(16),
            rej_wt: row.get(17),
            end_pc_wt: row.get(18),
            total_wt: row.get(19),
            issued_qty: row.get(20),
            created_by: row.get(21),
            created_on: row.get(22),
            created_login_key: row.get(23),
            modified_by: row.get(24),
            modified_on: row.get(25),
            modified_login_key: row.get(26)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Requisition>>)),
        _ => Json(json!(vector))
    }
}