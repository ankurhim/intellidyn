use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}, http::response::Response, http::StatusCode };
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFGMaterialRequest {
    pub fg_date: String,
    pub drawing_no: String,
    pub available_qty: i64,
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub forging_weight: f64,
    pub quality_status: Option<String>,
    pub batch_status: Option<String>,
    pub remarks: Option<String>
}

impl CreateFGMaterialRequest {
    pub async fn create_fg_material_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let service_response = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_fg_material_table (
                id SERIAL NOT NULL,
                fg_material_pk TEXT NOT NULL PRIMARY KEY,
                fg_date DATE NOT NULL,
                drawing_no TEXT NOT NULL,
                available_qty BIGINT NOT NULL,
                heat_no TEXT NOT NULL,
                grade TEXT NOT NULL,
                section BIGINT NOT NULL,
                section_type TEXT NOT NULL,
                forging_weight FLOAT8 NOT NULL,
                quality_status TEXT NOT NULL DEFAULT 'UNDER INSPECTION',
                batch_status TEXT NOT NULL DEFAULT 'NOT ISSUED',
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (fg_material_pk)
            )",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match service_response {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_fg_material_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let service_response = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_fg_material_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match service_response {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn truncate_fg_material_table(
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

        let service_response = service.client
        .execute(
            "TRUNCATE TABLE mwspl_fg_material_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match service_response {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_fg_material(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
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

        let fg_date = NaiveDate::parse_from_str(&payload.fg_date, "%d-%m-%Y").expect("PO Date parsing error");

        let service_response = service.client
        .execute(
            "INSERT INTO mwspl_fg_material_table (
                fg_material_pk,
                fg_date,
                drawing_no,
                available_qty,
                heat_no,
                grade,
                section,
                section_type,
                forging_weight,
                quality_status,
                batch_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)",
            &[
                &Uuid::new_v4().to_string(),
                &fg_date,
                &payload.drawing_no,
                &payload.available_qty,
                &payload.heat_no,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.forging_weight,
                &payload.quality_status,
                &payload.batch_status,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>,
                &payload.remarks
            ]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match service_response {
            Ok(v) => v,
            Err(e) => e
        }
    }
}