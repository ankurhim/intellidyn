pub mod cutting_request_model;
pub mod create_request;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_request::CreateSteelRequisitionRequest,
};

pub async fn create_steel_request_routes() -> Router {
    Router::new()
    .route("/create_new_steel_request", post(CreateSteelRequisitionRequest::create_new_requisition))
    .route("/drop_steel_request_table", get(CreateSteelRequisitionRequest::drop_steel_request_table))
}