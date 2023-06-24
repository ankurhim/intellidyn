use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}, http::StatusCode};
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::steels::steel_model::Steel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartRequest {
    pub part_code: String,
    pub part_no: String,
    pub part_name: String,
    pub dwg_rev_no: String,
    pub steel_code: String,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub cut_length: Option<f64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreatePartRequest {
    pub async fn create_part_table(
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
                return Json(json!(CreatePartResponse {
                    data: None,
                    error: Some("The user is not authorized".to_string())
                }));
            }
        }
        
        match service.client
        .execute("CREATE TABLE IF NOT EXISTS mwspl_part_table(
            id SERIAL NOT NULL,
            part_pk TEXT NOT NULL,
            part_code TEXT NOT NULL PRIMARY KEY,
            part_no TEXT NOT NULL,
            part_name TEXT NOT NULL,
            dwg_rev_no TEXT NOT NULL,
            steel_code TEXT NOT NULL REFERENCES mwspl_steel_table(steel_code) ON UPDATE NO ACTION ON DELETE NO ACTION,
            gross_weight FLOAT8 NOT NULL,
            cut_weight FLOAT8 NOT NULL,
            cut_length FLOAT8,
            created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
            created_on TIMESTAMPTZ NOT NULL,
            created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
            modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
            modified_on TIMESTAMPTZ,
            modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
            UNIQUE (part_code)
        );", &[])
        .await
        .map(|val| Json(json!(CreatePartResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_part_table(
        Path((user, login_key)): Path<(String, String)>,   
        Extension(service): Extension<Arc<DbService>>,
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
                return Json(json!(CreatePartResponse {
                    data: None,
                    error: Some("The user is not authorized".to_string())
                }));
            }
        }

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_part_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreatePartResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_part(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
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
                return Json(json!(CreatePartResponse {
                    data: None,
                    error: Some("The user is not authorized".to_string())
                }));
            }
        }

        match service.client
        .execute(
           "INSERT INTO mwspl_part_table(
            part_pk,
            part_code,
            part_no,
            part_name,
            dwg_rev_no,
            steel_code,
            gross_weight,
            cut_weight,
            cut_length,
            created_by,
            created_on,
            created_login_key,
            modified_by,
            modified_on,
            modified_login_key
           ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
           &[
            &Uuid::new_v4().to_string(),
            &payload.part_code,
            &payload.part_no,
            &payload.part_name,
            &payload.dwg_rev_no,
            &payload.steel_code,
            &payload.gross_weight,
            &payload.cut_weight,
            &payload.cut_length,
            &user,
            &Local::now(),
            &login_key,
            &None::<String>,
            &None::<DateTime<Local>>,
            &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(CreatePartResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }
}