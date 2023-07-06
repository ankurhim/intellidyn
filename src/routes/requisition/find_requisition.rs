use serde::{Serialize, Deserialize };
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
use crate::routes::requisition::requisition_model::Requisition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindRequisitionRequest {
    pub filter: Option<String>
}

impl FindRequisitionRequest {
    pub async fn find_requisition_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_requisition_table';",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn find_all_requisitions(
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
        .query("SELECT * FROM mwspl_requisition_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_all_requisitions_by_sender(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(payload): Query<FindRequisitionRequest>
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
        .query("SELECT * FROM mwspl_requisition_table WHERE request_from = $1;", &[&payload.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_all_requisitions_by_receiver(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(payload): Query<FindRequisitionRequest>
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
        .query("SELECT * FROM mwspl_requisition_table WHERE request_to = $1 AND status = 'OPEN';", &[&payload.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_open_requisitions(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,        
        Query(payload): Query<FindRequisitionRequest>
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
        .query("SELECT * FROM mwspl_requisition_table WHERE request_from = $1 AND status = 'OPEN';", &[&payload.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Requisition> = Vec::new();
    
    for row in row_vector {
        vector.push(Requisition {
            requisition_pk: row.get(1),
            request_from: row.get(2),
            request_to: row.get(3),
            part_no: row.get(4),
            requested_qty: row.get(5),
            comments: row.get(6),
            request_status: row.get(7),
            reply: row.get(8),
            created_by: row.get(9),
            created_on: row.get(10),
            created_login_key: row.get(11),
            modified_by: row.get(12),
            modified_on: row.get(13),
            modified_login_key: row.get(14)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Requisition>>)),
        _ => Json(json!(vector))
    }
}