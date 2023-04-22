use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::verify;
use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindUserRequest {
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Debug, Serialize)]
pub struct FindUserResponse {
    pub success: bool,
    pub data: Vec<User>,
    pub error: Option<String>
}

impl FindUserRequest {
    pub async fn find_users(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let mut user_vector: Vec<User> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_user", &[]
        )
        .await
        .map_err(|e| Json(json!(FindUserResponse {
            success: false,
            data: vec![],
            error: Some(e.to_string())
        })));

        for row in resp.unwrap() {
            user_vector.push(User {
                user_pk: Uuid::parse_str(row.get(1)).unwrap(),
                full_name: row.get(2),
                username: row.get(3),
                password: row.get(4),
                phone_no: row.get(5),
                created_by: row.get(6),
                created_on: row.get(7),
                modified_by: row.get(8),
                modified_on: row.get(9)
            })
        }

        Json(json!(FindUserResponse {
            success: true,
            data: user_vector,
            error: None,
        }))
    }

    pub async fn find_user_by_username(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindUserRequest>,
    ) -> Json<Value> {
        let mut user_vector: Vec<User> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_user WHERE username = $1", &[&query.username]
        )
        .await
        .map_err(|e| Json(json!(FindUserResponse {
            success: false,
            data: vec![],
            error: Some(e.to_string())
        })));

        for row in resp.unwrap() {
            user_vector.push(User {
                user_pk: Uuid::parse_str(row.get(1)).unwrap(),
                full_name: row.get(2),
                username: row.get(3),
                password: row.get(4),
                phone_no: row.get(5),
                created_by: row.get(6),
                created_on: row.get(7),
                modified_by: row.get(8),
                modified_on: row.get(9)
            })
        }

        Json(json!(FindUserResponse {
            success: true,
            data: user_vector,
            error: None,
        }))
    }
}