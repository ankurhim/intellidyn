pub mod users;
pub mod auth;
pub mod incoming_steel;
pub mod approved_components;
pub mod cutting_production;

use axum::{
    Router,
    Extension,
};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::users::create_user_routes;
use crate::routes::auth::create_auth_routes;
use crate::routes::users::user_model::User;
use crate::routes::incoming_steel::create_incoming_routes;
use crate::routes::approved_components::create_approved_component_routes;


pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let logged_user = Arc::new(User::default());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .nest("/auth/", create_auth_routes().await)
    .nest("/incoming_steels/", create_incoming_routes().await)
    .nest("/approved_components/", create_approved_component_routes().await)
    .layer(Extension(logged_user))
    .layer(Extension(client));

    routes
}