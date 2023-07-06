use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{Local, DateTime};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path,
    http::StatusCode,
};

use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::response::Response;

pub async fn authorize(
    Path((user, login_key)): Path<(String, String)>,
    Extension(service): Extension<Arc<DbService>>
) -> Response<String> {

    let mut response = Response::default();

    let resp = service.client
    .query(
        "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
    )
    .await;
    
    for row in resp.unwrap() {
        response = if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
            Response::<String> {
                status_code: StatusCode::CONTINUE,
                data: Some("Access Granted".to_string()),
                error: None,
                success: true
            }
        } else {
            Response::<String> {
                status_code: StatusCode::UNAUTHORIZED,
                data: None,
                error: Some("Access Denied".to_string()),
                success: true
            }
        }
    }

    response
}