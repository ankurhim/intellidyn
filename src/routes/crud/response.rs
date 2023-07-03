use serde::{Serialize, Deserialize};
use axum::http::StatusCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub data: Option<String>,
    pub error: Option<String>,
    pub success: bool
}