pub mod approved_components_model;
pub mod create_approvals;
// pub mod find_incoming;
// pub mod delete_user;
// pub mod update_user;

use axum::{
    Router,
    routing::{get, post, delete, put},
};

use self::{
    approved_components_model::ApprovedComponent,
    create_approvals::CreateApprovedComponentRequest,
    // find_incoming::FindIncomingSteelRequest,
    // delete_user::DeleteUserRequest,
    // update_user::UpdateUserRequest,
};

pub async fn create_approved_component_routes() -> Router {
    Router::new()
    .route("/create_new_approval", post(CreateApprovedComponentRequest::create_new_approved_component))
    // .route("/find_incoming_steels", get(FindIncomingSteelRequest::find_incoming_steels))
    // .route("/find_incoming_steels_by_heat_no", get(FindIncomingSteelRequest::find_incoming_steels_by_heat_no))
    // .route("/delete_user_by_username", delete(DeleteUserRequest::delete_user_by_username))
    // .route("/update_user_by_username", put(UpdateUserRequest::update_user_by_username))
}