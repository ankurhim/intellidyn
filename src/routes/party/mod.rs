pub mod party_model;
pub mod create_party;
// pub mod find_party;
// pub mod update_party;

use axum::{
    Router,
    routing::{post},
};

use self::{
    create_party::CreatePartyRequest,
    // find_party::FindPartyrRequest,
    // update_party::UpdatePartyrRequest
};

pub async fn create_party_routes() -> Router {
    Router::new()
    .route("/create_party_table", post(CreatePartyRequest::create_party_table))
    .route("/drop_party_table", post(CreatePartyRequest::drop_party_table))
    .route("/:user/:login_key/create_new_party", post(CreatePartyRequest::create_new_party))
    // .route("/:user/:login_key/find_all_partys", get(FindPartyrRequest::find_all_partys))
    // .route("/:user/:login_key/find_all_partys_by_dwg_no", get(FindPartyrRequest::find_all_partys_by_dwg_no))
    // .route("/:user/:login_key/find_active_partys", get(FindPartyrRequest::find_active_partys))
    // .route("/:user/:login_key/find_active_partys_by_dwg_no", get(FindPartyrRequest::find_active_partys_by_dwg_no))
    // .route("/:user/:login_key/update_po_status_by_filter", put(UpdatePartyrRequest::update_po_status_by_filter))
}