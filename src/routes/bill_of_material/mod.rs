pub mod bill_of_material_model;
pub mod create_bom;
pub mod find_bom;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_bom::CreateBillOfMaterialRequest,
    find_bom::FindBillOfMaterialRequest,
};

pub async fn create_bom_routes() -> Router {
    Router::new()
    .route("/create_table", post(CreateBillOfMaterialRequest::create_bom_table))
    .route("/drop_table", post(CreateBillOfMaterialRequest::drop_bom_table))
    .route("/create_new_bom", post(CreateBillOfMaterialRequest::create_bom))
    .route("/find_table", get(FindBillOfMaterialRequest::find_bom_table))
    .route("/find_all_boms", get(FindBillOfMaterialRequest::find_all_boms))
}