pub mod incoming_steel_model;
pub mod create_incoming;
pub mod find_incoming;
pub mod inventory;
pub mod delete_incoming;
// pub mod update_user;

use axum::{
    Router,
    routing::{get, post, delete},
};

use self::{
    incoming_steel_model::IncomingSteel,
    create_incoming::CreateIncomingSteelRequest,
    find_incoming::FindIncomingSteelRequest,
    inventory::{ FindInventoryRequest, FindInventoryByDateRangeRequest },
    delete_incoming::DeleteIncomingSteelRequest,
    // update_user::UpdateUserRequest,
};

pub async fn create_incoming_routes() -> Router {
    Router::new()
    .route("/create_new_incoming_steel", post(CreateIncomingSteelRequest::create_new_incoming_steel))
    .route("/find_incoming_steels", get(FindIncomingSteelRequest::find_incoming_steels))
    .route("/find_incoming_steels_by_filter", get(FindIncomingSteelRequest::find_incoming_steels_by_filter))
    .route("/get_inventory", get(FindInventoryRequest::get_inventory))
    .route("/get_inventory_by_filter", get(FindInventoryRequest::get_inventory_by_filter))
    .route("/get_inventory_by_date_range", get(FindInventoryByDateRangeRequest::get_inventory_by_date_range))
    .route("/delete_steel_by_filter", delete(DeleteIncomingSteelRequest::delete_steel_by_filter))
    // .route("/update_user_by_username", put(UpdateUserRequest::update_user_by_username))
}