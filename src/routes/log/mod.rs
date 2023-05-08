pub mod log_model;
pub mod create_log;
pub mod find_logs;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_log::CreateLogRequest,
    find_logs::FindLogRequest
};

pub async fn create_log_routes() -> Router {
    Router::new()
    .route("/create_log_table", post(CreateLogRequest::create_log_table))
    .route("/delete_log_table", post(CreateLogRequest::drop_log_table))
    .route("/create_new_log", post(CreateLogRequest::create_new_log))
    .route("/find_all_logs", get(FindLogRequest::find_logs))
    .route("/find_active_log_by_username", get(FindLogRequest::find_active_log_by_username))
    // .route("/find_logs", get(FindLogRequest::find_logs))
}