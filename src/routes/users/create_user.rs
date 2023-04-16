use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::time;
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
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
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
                username TEXT NOT NULL,
                password TEXT NOT NULL,
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
            error: Some(e.to_string())
        })));

        let new_user = User {
            user_pk: Uuid::new_v4(),
            username: payload.username.clone(),
            password: hash(payload.password.clone(), DEFAULT_COST).expect("Hashing failed"),
            created_by: Some(logged_user.username.to_string()),
            created_on: time::SystemTime::now(),
            modified_by: None,
            modified_on: None,
        };

        let resp = service.client
        .execute(
            "INSERT INTO intellidyn_user(
                user_pk,
                username,
                password,
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
                $7
            )", &[
                &new_user.user_pk.to_string(),
                &new_user.username,
                &new_user.password,
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
        .map(|_| Json(json!(CreateUserResponse {
            success: true,
            error: None
        })))
        .map_err(|e| Json(json!(CreateUserResponse {
            success: false,
            error: Some(e.to_string())
        })));

        resp.unwrap()
    }
}