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
            "SELECT * FROM mwspl_user_table WHERE username = $1", &[
                &payload.username
                ]
        )
        .await
        .map_err(|e| Json(json!(UserLoginResponse {
            data: None,
            error: Some(e.to_string())
        })));

        let login_result = if &query_result.as_ref().unwrap().len() == &1 {
            let result = &query_result.unwrap()[0];

            let user = User {
                user_pk: Uuid::parse_str(result.get(1)).unwrap(),
                full_name: result.get(2),
                employee_id: result.get(3),
                username: result.get(4),
                password: result.get(5),
                phone_no: result.get(6),
                created_by: result.get(7),
                created_on: result.get(8),
                modified_by: result.get(9),
                modified_on: result.get(10),
                remarks: result.get(11)
            };
    
            Json(json!(verify(payload.password.unwrap(), &user.password.unwrap()).unwrap()))
        } else {
            Json(json!(None::<bool>))
        };

        login_result
    }
}