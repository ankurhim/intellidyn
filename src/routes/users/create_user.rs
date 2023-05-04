use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{naive::NaiveDateTime, Utc};
use std::sync::Arc;
use bcrypt::{ hash, DEFAULT_COST };
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub username: String,
    pub password: String,
    pub phone_no: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateUserRequest {
    pub async fn create_new_user(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Json<Value> {
        
        let _create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS intellidyn_user (
                id SERIAL NOT NULL,
                user_pk TEXT NOT NULL,
                full_name TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                phone_no TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMP NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMP,
                UNIQUE (username)
            );", &[]
        ).
        await
        .map_err(|e| Json(json!(CreateUserResponse {
            success: false,
            data: None,
            error: Some(e.to_string())
        })));

        let new_user = User {
            user_pk: Uuid::new_v4(),
            full_name: payload.full_name.clone(),
            username: payload.username.clone(),
            password: hash(payload.password.clone(), DEFAULT_COST).expect("Hashing failed"),
            phone_no: match &payload.phone_no.len() {
                0 => Some(payload.phone_no),
                _ => None,
            },
            created_by: Some(logged_user.username.to_string()),
            created_on: Utc::now(),
            modified_by: None,
            modified_on: None,
        };

        let resp = service.client
        .execute(
            "INSERT INTO intellidyn_user(
                user_pk,
                full_name,
                username,
                password,
                phone_no,
                created_by,
                created_on,
                modified_by,
                modified_on
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9
            )", &[
                &new_user.user_pk.to_string(),
                &new_user.full_name,
                &new_user.username,
                &new_user.password,
                match &new_user.phone_no {
                    Some(v) => v,
                    _ => &None::<String>
                },
                match &new_user.created_by {
                    Some(v) => v,
                    _ => &None::<String>
                },
                &new_user.created_on,
                match &new_user.modified_by {
                    Some(v) => v,
                    _ => &None::<String>
                },
                &new_user.modified_on
            ]
        )
        .await
        .map(|val| Json(json!(CreateUserResponse {
            success: true,
            data: Some(format!("{:?}", val)),
            error: None
        })))
        .map_err(|e| Json(json!(CreateUserResponse {
            success: false,
            data: None,
            error: Some(e.to_string())
        })));

        match resp {
            Ok(v) => v,
            Err(e) => e
        }
    }
}