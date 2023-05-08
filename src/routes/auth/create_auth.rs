/// Dependencies imports
use uuid::Uuid;
use std::sync::Arc;
use axum::{Extension, Json};
use serde_json::{Value, json};
use serde::{Serialize, Deserialize };
use chrono::{DateTime, Local, NaiveDate};
///
/// Local crate imports
use crate::routes::User;
use crate::service::DbService;
use crate::routes::auth::auth_model::Auth;
///
/// Definition of user payload data for creating
/// new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthRequest {
    pub username: String,
    pub auths: Vec<String>,
    pub remarks: Option<String>
}
/// Definition of response for creating a new user
#[derive(Debug, Serialize)]
pub struct CreateAuthResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateAuthRequest {

    pub async fn create_auth_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let create_auth_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_auth_table (
                id SERIAL NOT NULL,
                auth_pk TEXT NOT NULL,
                username TEXT NOT NULL,
                auth TEXT NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (username, auth)
            );", &[]
        )
        .await
        .map(|val| Json(json!(CreateAuthResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateAuthResponse {
            data: None,
            error: Some(e.to_string())
        })));

        match create_auth_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_auth_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_auth_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_auth_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreateAuthResponse {
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(CreateAuthResponse {
            data: None,
            error: Some(e.to_string())
        })));

        drop_auth_table.unwrap()
    }

    pub async fn create_new_auth(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {

        let create_table = Self::create_auth_table(Extension(logged_user.clone()), Extension(service.clone())).await;

        let mut counter = 0;
        for auth in &payload.auths{
            service.client
            .execute(
                "INSERT INTO mwspl_auth_table(
                    auth_pk,
                    username,
                    auth,
                    created_by,
                    created_on,
                    modified_by,
                    modified_on,
                    remarks
                ) VALUES ($1, $2, #3, $4, $5, $6, $7, $8) INNER JOIN mwspl_user_table ON mwspl_user_table.username = mwspl_auth_table.username",
                &[
                    &Uuid::new_v4().to_string(),
                    &payload.username,
                    &auth,
                    &Some(logged_user.username.clone()),
                    &Local::now(),
                    &None::<String>,
                    &None::<DateTime<Local>>,
                    &None::<String>
                ]
            )
            .await
            .map(|val| {
                counter += 1;
                Json(json!(CreateAuthResponse {
                data: Some(format!("{:?}", val)),
                error: None,
            }))})
            .map_err(|e| {
                Json(json!(CreateAuthResponse {
                data: None,
                error: Some(e.as_db_error().unwrap().message().to_string())
            }))});
        }

        let result = if counter == payload.auths.len() {
            Json(json!(1))
        } else {
            Json(json!(0))
        };

        result
    }
}