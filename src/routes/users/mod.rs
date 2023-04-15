mod user_model;

use axum::{
    Router,
    routing::get,
    response::Json,
};
use serde_json::{Value, json};
use self::user_model::User;

pub async fn create_user_routes() -> Router {
    Router::new()
    .route("/get_users", get(json))
}


async fn json() -> Json<Value> {
    Json(json!(User::default()))
}