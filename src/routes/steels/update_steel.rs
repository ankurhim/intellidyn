use serde::{Serialize, Deserialize };
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path
};
use chrono::{DateTime, Local};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSteelRequest {
    pub steel_code: String,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: String
}

impl UpdateSteelRequest {
    pub async fn update_steel(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdateSteelRequest>,
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
            "UPDATE mwspl_steel_table
            SET steel_code = $1,
            SET steel_grade = $2,
            SET section = $3,
            SET section_type = $4,
            SET jominy_range = $5,
            SET modified_by = $6,
            SET modified_on = $7,
            SET modified_login_key = $8
            WHERE steel_code = $1", &[
                &payload.steel_code,
                &payload.steel_grade,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
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

    pub async fn update_steel_status(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdateSteelRequest>,
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
            "UPDATE mwspl_steel_table
            SET steel_status = 'DELETED',
            SET modified_by = $2,
            SET modified_on = $3,
            SET modified_login_key = $4
            WHERE steel_code = $1", &[
                &payload.steel_code,
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