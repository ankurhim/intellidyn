pub mod requisition_model;
pub mod create_requisition;
pub mod find_requisition;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_requisition::CreateRequisitionRequest,
    find_requisition::FindRequisitionRequest,
};

pub async fn create_requisition_routes() -> Router {
    Router::new()
    .route("/create_requisition_table", post(CreateRequisitionRequest::create_requisition_table))
    .route("/drop_requistion_table", post(CreateRequisitionRequest::drop_requisition_table))
    .route("/:user/:login_key/create_new_requistion", post(CreateRequisitionRequest::create_new_requisition))
    .route("/:user/:login_key/find_all_requistions", get(FindRequisitionRequest::find_all_requisitions))
    .route("/:user/:login_key/find_all_requistions_by_sender", get(FindRequisitionRequest::find_all_requisitions_by_sender))
    .route("/:user/:login_key/find_all_requistions_by_receiver", get(FindRequisitionRequest::find_all_requisitions_by_receiver))
}