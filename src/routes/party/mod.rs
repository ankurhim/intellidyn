pub mod party_model;
pub mod create_party;
pub mod find_party;
// pub mod update_party;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_party::CreatePartyRequest,
    find_party::FindPartyRequest,
};

pub async fn create_party_routes() -> Router {
    Router::new()
    .route("/create_party_table", post(CreatePartyRequest::create_party_table))
    .route("/drop_party_table", post(CreatePartyRequest::drop_party_table))
    .route("/:username/:login_key/create_new_party", post(CreatePartyRequest::create_new_party))
    .route("/:username/:login_key/find_all_parties", get(FindPartyRequest::find_all_parties))
}