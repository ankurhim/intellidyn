/// Dependencies imports
use uuid::Uuid;
use std::sync::Arc;
use axum::{Extension, Json, extract::Path};
use serde_json::{Value, json};
use bcrypt::{ hash, DEFAULT_COST };
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local};
///
/// Local crate imports
// use crate::error::AppError;
use crate::service::DbService;
///
/// Definition of user payload data for creating
/// new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub employee_id: String,
    pub username: String,
    pub password: String,
    pub email_id: Option<String>,
    pub role: String,
}

impl CreateUserRequest {

    pub async fn create_user_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_user_table (
                id SERIAL NOT NULL,
                user_pk TEXT NOT NULL,
                full_name TEXT NOT NULL,
                employee_id TEXT NOT NULL,
                username TEXT NOT NULL PRIMARY KEY,
                password TEXT NOT NULL,
                email_id TEXT,
                role TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (username)
            );", &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_user_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_user_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_user_table;", &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        drop_user_table.unwrap()
    }

    pub async fn create_new_user(
        // Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {

        // let resp = service.client
        // .query(
        //     "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        // )
        // .await
        // .map_err(|e| Json(json!(e.to_string())));

        // for row in resp.unwrap() {
        //     if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
        //         break;
        //     } else {
        //         return Json(json!("You are logged out"));
        //     }
        // }

        let hash = hash(&payload.password, DEFAULT_COST).expect("Hashing failed");

        match service.client
        .execute(
            "INSERT INTO mwspl_user_table(
                user_pk,
                full_name,
                employee_id,
                username,
                password,
                email_id,
                role,
                created_by,
                created_on,
                modified_by,
                modified_on,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.full_name,
                &payload.employee_id,
                &payload.username,
                &hash,
                &Some(payload.email_id),
                &payload.role,
                &"admin",
                &Local::now(),
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