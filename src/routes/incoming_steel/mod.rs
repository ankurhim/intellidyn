pub mod incoming_steel_model;
pub mod create_incoming;
pub mod find_incoming;
// pub mod inventory;
// pub mod delete_incoming;

use axum::{
    Router,
    routing::{get, post, delete},
};

use self::{
    create_incoming::CreateIncomingSteelRequest,
    find_incoming::FindIncomingSteelRequest,
    // inventory::{ FindInventoryRequest, FindInventoryByDateRangeRequest },
    // delete_incoming::DeleteIncomingSteelRequest,
};

pub async fn create_incoming_steel_routes() -> Router {
    Router::new()
    .route("/create_incoming_steel_table", post(CreateIncomingSteelRequest::create_incoming_steel_table))
    .route("/drop_incoming_steel_table", post(CreateIncomingSteelRequest::drop_incoming_steel_table))
    .route("/:user/:login_key/create_new_incoming_steel", post(CreateIncomingSteelRequest::create_new_incoming_steel))
    .route("/:user/:login_key/find_all_incoming_steels", get(FindIncomingSteelRequest::find_all_incoming_steels))
    .route("/:user/:login_key/find_all_incoming_steels_by_filter", get(FindIncomingSteelRequest::find_all_incoming_steels_by_filter))
}