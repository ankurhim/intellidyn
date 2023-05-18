pub mod fg_material_model;
pub mod create_fg_material;
pub mod find_fg_material;
pub mod inventory;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_fg_material::CreateFGMaterialRequest,
    find_fg_material::FindFGMaterialRequest,
};

pub async fn create_fg_material_routes() -> Router {
    Router::new()
    .route("/create_fg_material_table", post(CreateFGMaterialRequest::create_fg_material_table))
    .route("/drop_fg_material_table", post(CreateFGMaterialRequest::drop_fg_material_table))
    .route("/:user/:login_key/truncate_fg_material_table", post(CreateFGMaterialRequest::truncate_fg_material_table))
    .route("/:user/:login_key/create_new_fg_material", post(CreateFGMaterialRequest::create_new_fg_material))
    .route("/:user/:login_key/find_fg_material", get(FindFGMaterialRequest::find_fg_material))
    .route("/:user/:login_key/find_fg_material_by_dwg_no", get(FindFGMaterialRequest::find_fg_material_by_dwg_no))
}