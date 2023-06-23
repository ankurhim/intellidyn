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
use crate::routes::log::create_log::CreateLogRequest;
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
            "SELECT * FROM mwspl_user_table WHERE username = $1 ORDER BY created_on DESC LIMIT 1", &[
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
                email_id: result.get(6),
                role: result.get(7),
                created_by: result.get(8),
                created_on: result.get(9),
                modified_by: result.get(10),
                modified_on: result.get(11),
                remarks: result.get(12)
            };
    
            match verify(payload.password.unwrap(), &user.password.unwrap()).unwrap() {
                false => Json(json!(None::<bool>)),
                true => CreateLogRequest::create_new_log(Extension(service.clone()), Json(CreateLogRequest { username: user.username, login_key: Uuid::new_v4().to_string(), role: user.role })).await
            }
        } else {
            Json(json!(None::<bool>))
        };

        login_result
    }
}