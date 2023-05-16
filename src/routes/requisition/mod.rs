pub mod requisition_model;
pub mod create_requisition;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_requisition::CreateRequisitionRequest,
};

pub async fn create_requisition_routes() -> Router {
    Router::new()
    .route("/create_requisition_table", post(CreateRequisitionRequest::create_requisition_table))
    .route("/drop_requistion_table", post(CreateRequisitionRequest::drop_requisition_table))
}