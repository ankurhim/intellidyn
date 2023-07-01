use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMachineRequest {
    pub machine_id: String,
    pub machine_type: String,
    pub machine_name: String,
    pub machine_location: String,
    pub machine_model: String,
    pub machine_capacity: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMachineResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateMachineRequest {
    pub async fn create_machine_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_machine_table(
                id SERIAL NOT NULL,
                machine_pk TEXT NOT NULL,
                machine_id TEXT NOT NULL,
                machine_type TEXT NOT NULL,
                machine_name TEXT NOT NULL,
                machine_location TEXT NOT NULL,
                machine_model TEXT NOT NULL,
                machine_capacity TEXT NOT NULL,
                machine_status TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (machine_id)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateMachineResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateMachineResponse {
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_machine_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_machine_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateMachineResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateMachineResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_machine(
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
                return Json(json!("You are logged out"));
            }
        }
        
        match service.client
        .execute(
            "INSERT INTO mwspl_machine_table(
                machine_pk,
                machine_id,
                machine_type,
                machine_name,
                machine_location,
                machine_model,
                machine_capacity,
                machine_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.machine_id,
                &payload.machine_type,
                &payload.machine_name,
                &payload.machine_location,
                &payload.machine_model,
                &payload.machine_capacity,
                &None::<String>,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(CreateMachineResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateMachineResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }
}