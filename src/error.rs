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
    ObjectAlreadyExists,
    ObjectDoesNotExists
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occurred"
            ),
            Self::ObjectAlreadyExists => (StatusCode::BAD_REQUEST, "Object already exists"),
            Self::ObjectDoesNotExists => (StatusCode::BAD_REQUEST, "Object does not exist")
        };

        (status, Json(json!({"error": err_msg}))).into_response()
    }
}