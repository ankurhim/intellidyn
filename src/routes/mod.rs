pub mod users;
pub mod log;
pub mod party;
pub mod bill_of_material;
pub mod incoming_steel;
pub mod approved_components;

use axum::{Router, Extension};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::users::create_user_routes;
use crate::routes::log::create_log_routes;
use crate::routes::party::create_party_routes;
use crate::routes::bill_of_material::create_bill_of_material_routes;
use crate::routes::incoming_steel::create_incoming_steel_routes;
use crate::routes::approved_components::create_approved_component_routes;


pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .nest("/log/", create_log_routes().await)
    .nest("/party/", create_party_routes().await)
    .nest("/bill_of_material/", create_bill_of_material_routes().await)
    .nest("/incoming_steel/", create_incoming_steel_routes().await)
    .nest("/part_approvals/", create_approved_component_routes().await)
    .layer(Extension(client));

    routes
}