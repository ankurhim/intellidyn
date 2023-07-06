use serde::{Serialize, Deserialize };
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path
};
use chrono::{DateTime, Local};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub password: String
}

impl UpdateUserRequest {
    pub async fn change_password(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdateUserRequest>,
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

        let hash = hash(&payload.password, DEFAULT_COST).expect("Hashing failed");

        match service.client
        .execute(
            "UPDATE mwspl_user_table SET password = $2 WHERE username = $1", &[&user, &hash]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}