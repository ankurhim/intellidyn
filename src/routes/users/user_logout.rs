use serde::{Serialize, Deserialize };
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path
};
use chrono::Local;

use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogoutRequest {
    pub username: String,
    pub login_key: String
}

impl UserLogoutRequest {
    pub async fn user_logout(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        match service.client
        .execute(
            "UPDATE mwspl_log_table SET logout_time = $3 WHERE username = $1 AND login_key = $2 AND logout_time IS NULL", &[
                &user,
                &login_key,
                &Some(Local::now())
                ]
        )
        .await
        .map(|val| { println!("{}", &val); Json(json!(val))})
        .map_err(|e| { println!("{}", &e.to_string()); Json(json!(e.to_string()))}) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}