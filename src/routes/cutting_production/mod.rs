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
    .route("/create_steel_requisition_table", post(CreateSteelRequisitionRequest::create_steel_requisition_table))
    .route("/drop_steel_requisition_table", post(CreateSteelRequisitionRequest::drop_steel_requisition_table))
    .route("/:user/:login_key/create_new_steel_requisition", post(CreateSteelRequisitionRequest::create_new_steel_requisition))
//     .route("/drop_steel_request_table", get(CreateSteelRequisitionRequest::drop_steel_request_table))
}