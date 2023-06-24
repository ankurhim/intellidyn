pub mod steel_model;
pub mod create_steel;
pub mod find_steel;

use axum::{
    Router,
    routing::{post, get, put},
};

use self::{
    create_steel::CreateSteelRequest,
    find_steel::FindSteelRequest
};

pub async fn create_steel_routes() -> Router {
    Router::new()
    .route("/:username/:login_key/create_steel_table", post(CreateSteelRequest::create_steel_table))
    .route("/:username/:login_key/drop_steel_table", post(CreateSteelRequest::drop_steel_table))
    .route("/:username/:login_key/create_new_steel", post(CreateSteelRequest::create_new_steel))
    .route("/:username/:login_key/find_all_steels", get(FindSteelRequest::find_all_steels))
}