pub mod cutting_model;
pub mod create_cutting;

use axum::{
    Router,
    routing::{post, get, put},
};

use self::{
    create_cutting::CreateCuttingRequest,
};

pub async fn create_cutting_routes() -> Router {
    Router::new()
    .route("/create_cutting_temp_table", post(CreateCuttingRequest::create_cutting_temp_table))
    .route("/drop_cutting_table", post(CreateCuttingRequest::drop_cutting_table))
    // .route("/drop_bill_of_material_table", post(CreateBillOfMaterialRequest::drop_bill_of_material_table))
    // .route("/:user/:login_key/create_new_bill_of_material", post(CreateBillOfMaterialRequest::create_new_bill_of_material))
    // .route("/:user/:login_key/find_all_bill_of_materials", get(FindBillOfMaterialRequest::find_all_boms))
}