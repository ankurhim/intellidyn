pub mod bom_model;
pub mod create_bom;
pub mod find_bom;
pub mod update_bom;

use axum::{
    Router,
    routing::{post, get, put},
};

use self::{
    create_bom::CreateBillOfMaterialRequest,
    find_bom::FindBillOfMaterialRequest,
    update_bom::UpdateBillOfMaterialRequest
};

pub async fn create_bill_of_material_routes() -> Router {
    Router::new()
    .route("/create_bill_of_material_table", post(CreateBillOfMaterialRequest::create_bill_of_material_table))
    .route("/drop_bill_of_material_table", post(CreateBillOfMaterialRequest::drop_bill_of_material_table))
    .route("/:user/:login_key/create_new_bill_of_material", post(CreateBillOfMaterialRequest::create_new_bill_of_material))
    .route("/:user/:login_key/find_all_bill_of_materials", get(FindBillOfMaterialRequest::find_all_boms))
    .route("/:user/:login_key/find_all_bill_of_materials_by_dwg_no", get(FindBillOfMaterialRequest::find_all_boms_by_dwg_no))
    .route("/:user/:login_key/find_active_bill_of_materials", get(FindBillOfMaterialRequest::find_active_boms))
    .route("/:user/:login_key/find_active_bill_of_materials_by_dwg_no", get(FindBillOfMaterialRequest::find_active_boms_by_dwg_no))
    .route("/:user/:login_ket/find_all_dwg_no", get(FindBillOfMaterialRequest::find_all_dwg_no))
    .route("/:user/:login_key/update_po_status_by_filter", put(UpdateBillOfMaterialRequest::update_bom_status_by_filter))
}