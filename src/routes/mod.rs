pub mod users;
pub mod log;
pub mod party;
pub mod bill_of_material;
pub mod incoming_steel;
pub mod approved_components;
pub mod requisition;
pub mod schedule;
pub mod steels;
pub mod parts;

use axum::{Router, Extension};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::{
    users::create_user_routes,
    steels::create_steel_routes,
    parts::create_part_routes,
    log::create_log_routes,
    party::create_party_routes,
    bill_of_material::create_bill_of_material_routes,
    incoming_steel::create_incoming_steel_routes,
    approved_components::create_approved_component_routes,
    requisition::create_requisition_routes,
    schedule::create_schedule_routes,
};

pub async fn create_routes() -> Router {
    let client = Arc::new(DbService::new()
    .await
    .unwrap());

    let routes = Router::new()
    .nest("/users/", create_user_routes().await)
    .nest("/steels/",create_steel_routes().await)
    .nest("/parts/",create_part_routes().await)
    .nest("/log/", create_log_routes().await)
    .nest("/parties/", create_party_routes().await)
    .nest("/bill_of_material/", create_bill_of_material_routes().await)
    .nest("/incoming_steel/", create_incoming_steel_routes().await)
    .nest("/part_approvals/", create_approved_component_routes().await)
    .nest("/requisition/", create_requisition_routes().await)
    .nest("/schedule/", create_schedule_routes().await)
    .layer(Extension(client));

    routes
}