pub mod purchase_order_model;
pub mod create_purchase_order;
pub mod find_purchase_order;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_purchase_order::CreatePurchaseOrderRequest,
    find_purchase_order::FindPurchaseOrderRequest,
};

pub async fn create_purchase_order_routes() -> Router {
    Router::new()
    .route("/create_purchase_order_table", post(CreatePurchaseOrderRequest::create_purchase_order_table))
    .route("/drop_purchase_order_table", post(CreatePurchaseOrderRequest::drop_purchase_order_table))
    .route("/create_new_purchase_order", post(CreatePurchaseOrderRequest::create_new_purchase_order))
    .route("/find_all_purchase_orders", get(FindPurchaseOrderRequest::find_all_purchase_orders))
    .route("/find_all_purchase_orders_by_po_no", get(FindPurchaseOrderRequest::find_all_purchase_orders_by_po_no))
}