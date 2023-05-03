use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub enum AppError {
    InternalServerError,
    TableDoesNotExist,
    TableCreationFailed,
    DataInsertionFailed,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occurred"
            ),
            Self::TableDoesNotExist => (StatusCode::BAD_REQUEST, "Table does not exist"),
            Self::TableCreationFailed => (StatusCode::BAD_REQUEST, "Table creation failed"),
            Self::DataInsertionFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Data insertion failed")
        };

        (status, Json(json!({"error": err_msg}))).into_response()
    }
}