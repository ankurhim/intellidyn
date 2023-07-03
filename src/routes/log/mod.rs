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
    .route("/find_all_logs", get(FindLogRequest::find_logs))
    .route("/:username/:login_key/find_all_logs_by_username", get(FindLogRequest::find_logs_by_username))
    .route("/:username/:login_key/find_all_logs_by_username_filter_by_date", post(FindLogRequest::find_logs_by_username_filter_by_date))
    .route("/:username/:login_key/find_active_log_by_username", get(FindLogRequest::find_active_log_by_username))
}