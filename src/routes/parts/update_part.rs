use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use chrono::{DateTime, Local};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{Value, json};

use crate::routes::parts::part_model::Part;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePartRequest {
    pub part_code: String,
    pub dwg_rev_no: String,
    pub steel_code: String,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub cut_length: f64
}

impl UpdatePartRequest {
    pub async fn update_steel(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdatePartRequest>,
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
            "UPDATE mwspl_part_table
            SET dwg_rev_no = $2,
            SET steel_code = $3,
            SET gross_weight = $4,
            SET cut_weight = $5,
            SET cut_length = $6,
            SET modified_by = $7,
            SET modified_on = $8,
            SET modified_login_key = $9
            WHERE part_code = $1", &[
                &payload.part_code,
                &payload.dwg_rev_no,
                &payload.steel_code,
                &payload.gross_weight,
                &payload.cut_weight,
                &payload.cut_length,
                &user,
                &Local::now(),
                &login_key
                ]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn update_part_status(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdatePartRequest>,
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
            "UPDATE mwspl_part_table
            SET part_status = 'DELETED',
            SET modified_by = $2,
            SET modified_on = $3,
            SET modified_login_key = $4
            WHERE part_code = $1", &[
                &payload.part_code,
                &user,
                &Local::now(),
                &login_key
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