pub mod approved_components_model;
pub mod create_approvals;
pub mod find_approvals;
// pub mod delete_approvals;
// pub mod update_approvals;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_approvals::CreateApprovedComponentRequest,
    find_approvals::FindApprovedHeatsRequest,
    // delete_approvals::DeleteApprovedComponentRequest,
    // update_approvals::{UpdateApprovedComponentRequest, UpdateApprovedComponentTableRequest}
};

pub async fn create_approved_component_routes() -> Router {
    Router::new()
    .route("/create_approved_components_table", post(CreateApprovedComponentRequest::create_approved_components_table))
    .route("/drop_approved_components_table", post(CreateApprovedComponentRequest::drop_approved_components_table))
    .route("/:user/:login_key/create_new_approval", post(CreateApprovedComponentRequest::create_new_approved_components))
    .route("/:user/:login_key/find_approved_heats", get(FindApprovedHeatsRequest::find_approved_heats))
    .route("/:user/:login_key/find_approved_heats_by_filter", get(FindApprovedHeatsRequest::find_approved_heats_by_filter))
    // .route("/delete_part_by_filter", delete(DeleteApprovedComponentRequest::delete_part_by_filter))
    // .route("/update_approvals", put(UpdateApprovedComponentRequest::update_approved_component_by_heat_no))
    // .route("/alter_approval_table", put(UpdateApprovedComponentTableRequest::update_section_by_heat_no))
    // .route("/alter_approved_component_table", post(CreateApprovedComponentRequest::alter_approved_component_table))
}