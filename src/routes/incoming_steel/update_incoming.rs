use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl UpdateUserRequest {
    pub async fn update_user_by_username(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(query): Json<UpdateUserRequest>,
    ) -> Json<Value> {
        let resp = service.client
        .execute(
            "UPDATE intellidyn_user SET password = $2, modified_by = $3, modified_on = $4 WHERE username = $1;", &[
                &query.username,
                &hash(query.password, DEFAULT_COST).expect("Hashing failed"),
                &logged_user.username,
                &Some(std::time::SystemTime::now())
                ]
        )
        .await
        .map(|val| Json(json!(UpdateUserResponse {
            success: true,
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(UpdateUserResponse {
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