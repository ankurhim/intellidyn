use axum::{
    routing::{get, post, delete, put},
    Router,
    Extension,
    response::Json,
};
use std::sync::Arc;
use serde_json::{Value, json};

use crate::service::DbService;

pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .route("/", get(json))
    .layer(Extension(client));

    routes
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42.20 }))
}