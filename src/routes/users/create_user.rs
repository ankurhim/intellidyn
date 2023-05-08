/// Dependencies imports
use uuid::Uuid;
use std::sync::Arc;
use axum::{Extension, Json};
use serde_json::{Value, json};
use bcrypt::{ hash, DEFAULT_COST };
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local, NaiveDate};
///
/// Local crate imports
// use crate::error::AppError;
use crate::service::DbService;
use crate::routes::users::user_model::User;
///
/// Definition of user payload data for creating
/// new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub employee_id: String,
    pub username: String,
    pub password: String,
    pub phone_no: Option<String>
}
/// Definition of response for creating a new user
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateUserRequest {

    pub async fn create_user_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        let drop_table = Self::drop_user_table(Extension(logged_user.clone()), Extension(service.clone())).await;

        let create_user_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_user_table (
                id SERIAL NOT NULL,
                user_pk TEXT NOT NULL,
                full_name TEXT NOT NULL,
                employee_id TEXT NOT NULL,
                username TEXT NOT NULL PRIMARY KEY,
                password TEXT NOT NULL,
                phone_no TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (username)
            );", &[]
        )
        .await
        .map(|val| Json(json!(CreateUserResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateUserResponse {
            data: None,
            error: Some(e.to_string())
        })));

        match create_user_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_user_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_user_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_user_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreateUserResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateUserResponse {
            data: None,
            error: Some(e.to_string())
        })));

        drop_user_table.unwrap()
    }

    pub async fn create_new_user(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {

        let create_table = Self::create_user_table(Extension(logged_user.clone()), Extension(service.clone())).await;

        let hash = hash(&payload.password, DEFAULT_COST).expect("Hashing failed");

        let result = service.client
        .execute(
            "INSERT INTO mwspl_user_table(
                user_pk,
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
                &Some(logged_user.username.clone()),
                &Local::now(),
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(CreateUserResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| {
            Json(json!(CreateUserResponse {
            data: None,
            error: Some(e.as_db_error().unwrap().message().to_string())
        }))});

        match result {
            Ok(v) => v,
            Err(e) => e
        }
    }
}