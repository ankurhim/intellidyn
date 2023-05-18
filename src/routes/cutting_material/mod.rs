pub mod cutting_material_model;
pub mod create_cutting_material;
pub mod find_cutting_material;
pub mod inventory;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_cutting_material::CreateCuttingMaterialRequest,
    find_cutting_material::FindCuttingMaterialRequest,
    inventory::Inventory
};

pub async fn create_cutting_material_routes() -> Router {
    Router::new()
    .route("/create_cutting_material_table", post(CreateCuttingMaterialRequest::create_cutting_material_table))
    .route("/drop_cutting_material_table", post(CreateCuttingMaterialRequest::drop_cutting_material_table))
    .route("/:user/:login_key/truncate_cutting_material_table", post(CreateCuttingMaterialRequest::truncate_cutting_material_table))
    .route("/:user/:login_key/create_new_cutting_material", post(CreateCuttingMaterialRequest::create_new_cutting_material))
    .route("/:user/:login_key/find_cutting_material", get(FindCuttingMaterialRequest::find_cutting_material))
    .route("/:user/:login_key/find_cutting_material_by_dwg_no", get(FindCuttingMaterialRequest::find_cutting_material_by_dwg_no))
    .route("/:user/:login_key/inventory",get(Inventory::fetch_inventory))
    .route("/:user/:login_key/inventory_by_filter", get(Inventory::fetch_inventory_by_filter))
    .route("/:user/:login_key/inventory_by_date_range", get(Inventory::fetch_inventory_filtered_by_date))
}