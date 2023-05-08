/// Dependencies imports
use uuid::Uuid;
use std::sync::Arc;
use axum::{Extension, Json,extract::Query};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local, NaiveDate};
///
/// Local crate imports
// use crate::error::AppError;
use crate::service::DbService;
use crate::routes::users::user_model::User;
use crate::routes::log::find_logs::FindLogRequest;
///
/// Definition of user payload data for creating
/// new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogRequest {
    pub username: String,
    pub login_key: String
}

impl CreateLogRequest {

    pub async fn create_log_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        let drop_table = Self::drop_log_table(Extension(service.clone())).await;

        let create_log_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_log_table (
                id SERIAL NOT NULL,
                log_pk TEXT NOT NULL,
                username TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                login_key TEXT NOT NULL,
                login_time TIMESTAMPTZ NOT NULL,
                logout_time TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (username, login_key)
            );", &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match create_log_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_log_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_log_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_log_table;", &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        drop_log_table.unwrap()
    }

    pub async fn create_new_log(
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {
        let result = service.client
        .execute(
            "INSERT INTO mwspl_log_table(
                log_pk,
                username,
                login_key,
                login_time,
                logout_time,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.username,
                &payload.login_key,
                &Local::now(),
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| async {
            let query = FindLogRequest {username: Some(payload.username) };
            FindLogRequest::find_active_log_by_username(Extension(service.clone()), Query(query)).await
        })
        .map_err(|e| Json(json!(e.as_db_error().unwrap().message().to_string())));

        match result {
            Ok(v) => v.await,
            Err(e) => e
        }
    }
}