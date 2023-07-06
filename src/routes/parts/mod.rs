pub mod part_model;
pub mod create_part;
pub mod find_parts;

use axum::{
    Router,
    routing::{post, get, put},
};

use self::{
    create_part::CreatePartRequest,
    find_parts::FindPartRequest
};

pub async fn create_part_routes() -> Router {
    Router::new()
    .route("/create_part_table", post(CreatePartRequest::create_part_table))
    .route("/drop_part_table", post(CreatePartRequest::drop_part_table))
    .route("/:user/:login_key/create_new_part", post(CreatePartRequest::create_new_part))
    .route("/:user/:login_key/upload_part_csv", post(CreatePartRequest::upload_part_csv))
    .route("/:user/:login_key/find_all_parts", get(FindPartRequest::find_all_parts))
}