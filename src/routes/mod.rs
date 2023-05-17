pub mod users;
pub mod log;
pub mod party;
pub mod bill_of_material;
pub mod incoming_steel;
pub mod approved_components;
pub mod requisition;
pub mod issue_material;
pub mod cutting_store;

use axum::{Router, Extension};
use std::sync::Arc;

use crate::service::DbService;
use crate::routes::{
    users::create_user_routes,
    log::create_log_routes,
    party::create_party_routes,
    bill_of_material::create_bill_of_material_routes,
    incoming_steel::create_incoming_steel_routes,
    approved_components::create_approved_component_routes,
    requisition::create_requisition_routes,
    issue_material::create_material_issue_routes,
    cutting_store::create_cutting_inventory_routes,
};


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
    .nest("/requisition/", create_requisition_routes().await)
    .nest("/material_issue/", create_material_issue_routes().await)
    .nest("/cutting_store", create_cutting_inventory_routes().await)
    .layer(Extension(client));

    routes
}