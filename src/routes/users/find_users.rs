use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use chrono::{DateTime, Local};

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
                email_id: row.get(6),
                role: row.get(7),
                created_by: row.get(8),
                created_on: row.get(9),
                modified_by: row.get(10),
                modified_on: row.get(11),
                remarks: row.get(12)
            })
        }

        Json(json!(user_vector))
    }

    pub async fn find_user_by_username(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindUserRequest>,
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
                password: None::<String>,
                email_id: row.get(6),
                role: row.get(7),
                created_by: row.get(8),
                created_on: row.get(9),
                modified_by: row.get(10),
                modified_on: row.get(11),
                remarks: row.get(12)
            })
        }

        Json(json!(user_vector))
    }
}