pub mod issue_material_model;
pub mod create_issued_material;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_issued_material::CreateMaterialIssueRequest
};

pub async fn create_material_issue_routes() -> Router {
    Router::new()
    .route("/create_material_issue_table", post(CreateMaterialIssueRequest::create_material_issue_table))
    .route("/drop_material_issue_table", post(CreateMaterialIssueRequest::drop_material_issue_table))
    .route("/:user/:login_key/create_new_material_issue", post(CreateMaterialIssueRequest::create_new_material_issue))
}