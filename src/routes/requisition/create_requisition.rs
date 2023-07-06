use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::Path};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequisitionRequest {
    pub request_from: String,
    pub request_to: String,
    pub part_no: String,
    pub requested_qty: i64,
    pub comments: Option<String>,
    pub request_status: String,
    pub reply:Option<String>
}

impl CreateRequisitionRequest {
    pub async fn create_requisition_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_requisition_table (
                id SERIAL NOT NULL PRIMARY KEY,
                requisition_id TEXT NOT NULL,
                request_from TEXT NOT NULL,
                request_to TEXT NOT NULL,
                part_no TEXT NOT NULL,
                requested_qty BIGINT NOT NULL,
                comments TEXT,
                request_status TEXT NOT NULL DEFAULT 'OPEN',
                reply TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (requisition_id, part_no)
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

    pub async fn drop_requisition_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_requisition_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_requisition(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<CreateRequisitionRequest>
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
            "INSERT INTO mwspl_requisition_table(
                requisition_pk,
                request_from,
                request_to,
                part_no,
                requested_qty,
                comments,
                request_status,
                reply,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.request_from,
                &payload.request_to,
                &payload.part_no,
                &payload.requested_qty,
                &payload.comments,
                &payload.request_status,
                &payload.reply,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
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