pub mod rm_inventory_model;
pub mod create_inventory;

use axum::{
    Router,
    routing::{get},
};

use self::{
    create_inventory::CreateRMInventoryRequest,
};

pub async fn create_inventory_routes() -> Router {
    Router::new()
    .route("/:user/:login_key/create_rm_inventory_routes", get(CreateRMInventoryRequest::create_rm_inventory))
}