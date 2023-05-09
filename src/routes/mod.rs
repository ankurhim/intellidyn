pub mod users;
pub mod auth;
pub mod purchase_order;
pub mod log;
// pub mod bill_of_material;
pub mod incoming_steel;
// pub mod approved_components;
// pub mod cutting_production;

use axum::{
    Router,
    Extension,
};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::users::create_user_routes;
use crate::routes::auth::create_auth_routes;
use crate::routes::users::user_model::User;
use crate::routes::purchase_order::create_purchase_order_routes;
use crate::routes::log::create_log_routes;
// use crate::routes::incoming_steel::create_incoming_routes;
// use crate::routes::approved_components::create_approved_component_routes;
// use crate::routes::cutting_production::create_steel_request_routes;
// use crate::routes::bill_of_material::create_bom_routes;


pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .nest("/auth/", create_auth_routes().await)
    .nest("/log/", create_log_routes().await)
    .nest("/purchase_order/", create_purchase_order_routes().await)
    // .nest("/incoming_steels/", create_incoming_routes().await)
    // .nest("/approved_components/", create_approved_component_routes().await)
    // .nest("/cutting_production", create_steel_request_routes().await)
    // .nest("/bill_of_material", create_bom_routes().await)
    .layer(Extension(client));

    routes
}