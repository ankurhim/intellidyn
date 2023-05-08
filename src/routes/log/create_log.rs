/// Dependencies imports
use uuid::Uuid;
use std::sync::Arc;
use axum::{Extension, Json};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local, NaiveDate};
///
/// Local crate imports
// use crate::error::AppError;
use crate::service::DbService;
use crate::routes::users::log_model::User;
///
/// Definition of user payload data for creating
/// new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogRequest {
    pub username: String
}
/// Definition of response for creating a new user
#[derive(Debug, Serialize)]
pub struct CreateLogResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateLogRequest {

    pub async fn create_log_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        let drop_table = Self::drop_log_table(Extension(logged_log.clone()), Extension(service.clone())).await;

        let create_log_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_log_table (
                id SERIAL NOT NULL,
                log_pk TEXT NOT NULL,
                username TEXT NOT NULL PRIMARY KEY,
                login_time TIMESTAMPTZ NOT NULL,
                logout_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (username, login_time)
            );", &[]
        )
        .await
        .map(|val| Json(json!(CreateLogResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateLogResponse {
            data: None,
            error: Some(e.to_string())
        })));

        match create_log_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_log_table(
        Extension(logged_log): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_log_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_log_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreateLogResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateLogResponse {
            data: None,
            error: Some(e.to_string())
        })));

        drop_log_table.unwrap()
    }

    pub async fn create_new_log(
        Extension(logged_log): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {

        let create_table = Self::create_log_table(Extension(logged_log.clone()), Extension(service.clone())).await;

        let hash = hash(&payload.password, DEFAULT_COST).expect("Hashing failed");

        let result = service.client
        .execute(
            "INSERT INTO mwspl_log_table(
                log_pk,
                full_name,
                employee_id,
                username,
                password,
                phone_no,
                created_by,
                created_on,
                modified_by,
                modified_on,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.full_name,
                &payload.employee_id,
                &payload.username,
                &hash,
                &Some(payload.phone_no),
                &Some(logged_log.username.clone()),
                &Local::now(),
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(CreateLogResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| {
            Json(json!(CreateLogResponse {
            data: None,
            error: Some(e.as_db_error().unwrap().message().to_string())
        }))});

        match result {
            Ok(v) => v,
            Err(e) => e
        }
    }
}