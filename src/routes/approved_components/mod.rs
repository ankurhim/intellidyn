pub mod approved_components_model;
pub mod create_approvals;
pub mod find_approvals;
pub mod delete_approvals;
// pub mod update_user;

use axum::{
    Router,
    routing::{get, post, delete, put},
};

use self::{
    approved_components_model::ApprovedComponent,
    create_approvals::CreateApprovedComponentRequest,
    find_approvals::FindApprovedHeatsRequest,
    delete_approvals::DeleteApprovedComponentRequest,
    // update_user::UpdateUserRequest,
};

pub async fn create_approved_component_routes() -> Router {
    Router::new()
    .route("/create_new_approval", post(CreateApprovedComponentRequest::create_new_approved_component))
    .route("/find_approved_heats", get(FindApprovedHeatsRequest::find_approved_heats))
    .route("/find_approved_heats_by_filter", get(FindApprovedHeatsRequest::find_incoming_steels_by_filter))
    .route("/delete_part_by_filter", delete(DeleteApprovedComponentRequest::delete_part_by_filter))
    // .route("/update_user_by_username", put(UpdateUserRequest::update_user_by_username))
}