pub mod users;
pub mod log;
pub mod purchase_order;

use axum::{Router, Extension};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::users::create_user_routes;
use crate::routes::purchase_order::create_purchase_order_routes;
use crate::routes::log::create_log_routes;


pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .nest("/log/", create_log_routes().await)
    .nest("/purchase_order/", create_purchase_order_routes().await)
    .layer(Extension(client));

    routes
}