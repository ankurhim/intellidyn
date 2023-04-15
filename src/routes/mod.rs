mod users;

use axum::{
    routing::{get, post, delete, put},
    Router,
    Extension,
    response::Json,
};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::users::create_user_routes;


pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .layer(Extension(client));

    routes
}