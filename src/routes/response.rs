use serde::{ Serialize, Deserialize};
use http_serde;
use axum::http;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub data: Option<T>,
    pub error: Option<String>,
    pub success: bool
}

impl<T> Response<T>
where
T: std::fmt::Debug + Clone + Serialize + for<'a>Deserialize<'a> + Sync + Send + tokio_postgres::types::ToSql
{
    pub fn default() -> Self {
        Response {
            status_code: http::StatusCode::NOT_FOUND,
            data: None,
            error: Some("Page is not found".to_string()),
            success: false
        }
    }
}