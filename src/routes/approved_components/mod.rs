pub mod approved_components_model;
pub mod create_approvals;
pub mod find_approvals;
pub mod delete_approvals;
pub mod update_approvals;

use axum::{
    Router,
    routing::{get, post, delete, put},
};

use self::{
    approved_components_model::ApprovedComponent,
    create_approvals::CreateApprovedComponentRequest,
    find_approvals::FindApprovedHeatsRequest,
    delete_approvals::DeleteApprovedComponentRequest,
    update_approvals::{UpdateApprovedComponentRequest, UpdateApprovedComponentTableRequest}
};

pub async fn create_approved_component_routes() -> Router {
    Router::new()
    .route("/create_new_approval", post(CreateApprovedComponentRequest::create_new_approved_component))
    .route("/find_approved_heats", get(FindApprovedHeatsRequest::find_approved_heats))
    .route("/find_approved_heats_by_filter", get(FindApprovedHeatsRequest::find_incoming_steels_by_filter))
    .route("/delete_part_by_filter", delete(DeleteApprovedComponentRequest::delete_part_by_filter))
    .route("/update_approvals", put(UpdateApprovedComponentRequest::update_approved_component_by_heat_no))
    .route("/alter_approval_table", put(UpdateApprovedComponentTableRequest::update_section_by_heat_no))
    .route("/alter_approved_component_table", post(CreateApprovedComponentRequest::alter_approved_component_table))
}