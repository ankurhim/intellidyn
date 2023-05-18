pub mod ht_material_model;
pub mod create_ht_material;
pub mod find_ht_material;
pub mod inventory;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_ht_material::CreateHTMaterialRequest,
    find_ht_material::FindHTMaterialRequest,
};

pub async fn create_ht_material_routes() -> Router {
    Router::new()
    .route("/create_ht_material_table", post(CreateHTMaterialRequest::create_ht_material_table))
    .route("/drop_ht_material_table", post(CreateHTMaterialRequest::drop_ht_material_table))
    .route("/:user/:login_key/truncate_ht_material_table", post(CreateHTMaterialRequest::truncate_ht_material_table))
    .route("/:user/:login_key/create_new_ht_material", post(CreateHTMaterialRequest::create_new_ht_material))
    .route("/:user/:login_key/find_ht_material", get(FindHTMaterialRequest::find_ht_material))
    .route("/:user/:login_key/find_ht_material_by_dwg_no", get(FindHTMaterialRequest::find_ht_material_by_dwg_no))
}