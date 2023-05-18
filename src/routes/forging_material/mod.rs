pub mod forging_material_model;
pub mod create_forging_material;
pub mod find_forging_material;
pub mod inventory;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_forging_material::CreateForgingMaterialRequest,
    find_forging_material::FindForgingMaterialRequest,
};

pub async fn create_forging_material_routes() -> Router {
    Router::new()
    .route("/create_forging_material_table", post(CreateForgingMaterialRequest::create_forging_material_table))
    .route("/drop_forging_material_table", post(CreateForgingMaterialRequest::drop_forging_material_table))
    .route("/:user/:login_key/truncate_forging_material_table", post(CreateForgingMaterialRequest::truncate_forging_material_table))
    .route("/:user/:login_key/create_new_forging_material", post(CreateForgingMaterialRequest::create_new_forging_material))
    .route("/:user/:login_key/find_forging_material", get(FindForgingMaterialRequest::find_forging_material))
    .route("/:user/:login_key/find_forging_material_by_dwg_no", get(FindForgingMaterialRequest::find_forging_material_by_dwg_no))
}