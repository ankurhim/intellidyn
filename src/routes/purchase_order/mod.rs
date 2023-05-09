pub mod purchase_order_model;
pub mod create_purchase_order;
pub mod find_purchase_order;
pub mod update_purchase_order;

use axum::{
    Router,
    routing::{post, get, put},
};

use self::{
    create_purchase_order::CreatePurchaseOrderRequest,
    find_purchase_order::FindPurchaseOrderRequest,
    update_purchase_order::UpdatePurchaseOrderRequest
};

pub async fn create_purchase_order_routes() -> Router {
    Router::new()
    .route("/create_purchase_order_table", post(CreatePurchaseOrderRequest::create_purchase_order_table))
    .route("/drop_purchase_order_table", post(CreatePurchaseOrderRequest::drop_purchase_order_table))
    .route("/:user/:login_key/create_new_purchase_order", post(CreatePurchaseOrderRequest::create_new_purchase_order))
    .route("/:user/:login_key/find_all_purchase_orders", get(FindPurchaseOrderRequest::find_all_purchase_orders))
    .route("/:user/:login_key/find_all_purchase_orders_by_dwg_no", get(FindPurchaseOrderRequest::find_all_purchase_orders_by_dwg_no))
    .route("/:user/:login_key/find_active_purchase_orders", get(FindPurchaseOrderRequest::find_active_purchase_orders))
    .route("/:user/:login_key/find_active_purchase_orders_by_dwg_no", get(FindPurchaseOrderRequest::find_active_purchase_orders_by_dwg_no))
    .route("/:user/:login_key/update_po_status_by_filter", put(UpdatePurchaseOrderRequest::update_po_status_by_filter))
}