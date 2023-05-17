pub mod cutting_store_model;
pub mod create_cutting_inventory;
pub mod find_cutting_inventory;

use axum::{
    Router,
    routing::{get, post},
};

use self::{
    create_cutting_inventory::CreateCuttingInventoryRequest,
    find_cutting_inventory::FindCuttingInventoryRequest,
};

pub async fn create_cutting_inventory_routes() -> Router {
    Router::new()
    .route("/create_cutting_inventory_table", post(CreateCuttingInventoryRequest::create_cutting_inventory_table))
    .route("/drop_cutting_inventory_table", post(CreateCuttingInventoryRequest::drop_cutting_inventory_table))
    .route("/:user/:login_key/truncate_cutting_inventory_table", post(CreateCuttingInventoryRequest::truncate_cutting_inventory_table))
    .route("/:user/:login_key/create_new_cutting_inventory", post(CreateCuttingInventoryRequest::create_new_cutting_inventory))
    .route("/:user/:login_key/find_cutting_inventory", get(FindCuttingInventoryRequest::find_cutting_inventory))
    .route("/:user/:login_key/find_cutting_inventory_by_dwg_no", get(FindCuttingInventoryRequest::find_cutting_inventory_by_dwg_no))
}