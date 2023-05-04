use serde::{Serialize, Deserialize };
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub username: Option<String>
}

#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl DeleteUserRequest {
    pub async fn delete_user_by_username(
        Extension(_logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<DeleteUserRequest>,
    ) -> Json<Value> {
        let resp = service.client
        .execute(
            "DELETE FROM intellidyn_user WHERE username = $1", &[&query.username]
        )
        .await
        .map(|val| Json(json!(DeleteUserResponse {
            success: true,
            data: Some(format!("{:?}", val)),
            error: None,
        })))
        .map_err(|e| Json(json!(DeleteUserResponse {
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