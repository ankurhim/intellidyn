use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path, Query}};
use serde_json::{Value, json};
use std::ops::Deref;

use crate::service::DbService;
use crate::routes::incoming_steel::update_incoming::UpdateInventoryRequest;
use crate::routes::cutting_material::find_cutting_material::FindCuttingMaterialRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialIssueRequest {
    pub material_issue_pk: Uuid,
    pub request_id: String,
    pub request_from: String,
    pub part_no: String,
    pub issued_qty: i64,
    pub reply: Option<String>
}

impl CreateMaterialIssueRequest {
    pub async fn create_material_issue_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_material_issue_table (
                id SERIAL NOT NULL PRIMARY KEY,
                material_issue_pk TEXT NOT NULL,
                request_id TEXT NOT NULL PRIMARY KEY,
                request_from TEXT NOT NULL,
                part_no TEXT NOT NULL,
                issued_qty TEXT NOT NULL,
                reply TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (request_id)
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

    pub async fn drop_material_issue_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_material_issue_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_material_issue(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<CreateMaterialIssueRequest>
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
            "INSERT INTO mwspl_material_issue_table(
                material_issue_pk,
                request_id,
                request_from,
                part_no,
                issued_qty,
                reply,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.request_id,
                &payload.request_from,
                &payload.part_no,
                &payload.issued_qty,
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
        .map(|val| async move {
            let update_inv = UpdateInventoryRequest{ drawing_no: payload.part_no, issued_qty: payload.issued_qty};
            UpdateInventoryRequest::update_inventory(Extension(service.clone()), Json(update_inv)).await;
            Json(json!(val))
        })
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => { return v.await; },
            Err(e) => { return e; }
        }
    }
}