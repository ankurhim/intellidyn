use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
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
    pub data: Vec<User>,
    pub error: Option<String>
}

impl FindUserRequest {
    pub async fn find_users(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let mut user_vector: Vec<User> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_user_table", &[]
        )
        .await
        .map_err(|e| Json(json!(FindUserResponse {
            data: vec![],
            error: Some(e.to_string())
        })));

        for row in resp.unwrap() {
            user_vector.push(User {
                user_pk: Uuid::parse_str(row.get(1)).unwrap(),
                full_name: row.get(2),
                employee_id: row.get(3),
                username: row.get(4),
                password: None::<String>,
                phone_no: row.get(6),
                created_by: row.get(7),
                created_on: row.get(8),
                modified_by: row.get(9),
                modified_on: row.get(10),
                remarks: row.get(11)
            })
        }

        Json(json!(user_vector))
    }

    pub async fn find_user_by_username(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindUserRequest>,
    ) -> Json<Value> {
        let mut user_vector: Vec<User> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_user_table WHERE username = $1", &[&query.username]
        )
        .await
        .map_err(|e| Json(json!(FindUserResponse {
            data: vec![],
            error: Some(e.to_string())
        })));

        for row in resp.unwrap() {
            user_vector.push(User {
                user_pk: Uuid::parse_str(row.get(1)).unwrap(),
                full_name: row.get(2),
                employee_id: row.get(3),
                username: row.get(4),
                password: row.get(5),
                phone_no: row.get(6),
                created_by: row.get(7),
                created_on: row.get(8),
                modified_by: row.get(9),
                modified_on: row.get(10),
                remarks: row.get(11)
            })
        }

        Json(json!(FindUserResponse {
            data: user_vector,
            error: None,
        }))
    }
}