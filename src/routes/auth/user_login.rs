use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::verify;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    pub success: bool,
    pub data: Option<User>,
    pub error: Option<String>
}

impl UserLoginRequest {
    pub async fn user_login(
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UserLoginRequest>,
    ) -> Json<Value> {
        let query_result = service.client
        .query(
            "SELECT * FROM intellidyn_user WHERE username = $1", &[
                &payload.username
                ]
        )
        .await
        .map_err(|e| Json(json!(UserLoginResponse {
            success: false,
            data: None,
            error: Some(e.to_string())
        })));

        let result = &query_result.unwrap()[0];

        let user = User {
            user_pk: Uuid::parse_str(result.get(1)).unwrap(),
            username: result.get(2),
            password: result.get(3),
            created_by: result.get(4),
            created_on: result.get(5),
            modified_by: result.get(6),
            modified_on: result.get(7)
        };

        let login_result = match verify(payload.password.unwrap(), &user.password).unwrap() {
            true => Json(json!(UserLoginResponse {
                success: true,
                data: Some(user),
                error: None
            })),
            false => Json(json!(UserLoginResponse {
                success: false,
                data: None,
                error: None
            })),
        };

        login_result
    }
}