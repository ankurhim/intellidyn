pub mod auth_model;
pub mod create_auth;
pub mod find_auths;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_auth::CreateAuthRequest,
    find_auths::FindAuthRequest,
};

pub async fn create_auth_routes() -> Router {
    Router::new()
    .route("/create_auth_table", post(CreateAuthRequest::create_auth_table))
    .route("/delete_auth_table", post(CreateAuthRequest::drop_auth_table))
    .route("/create_new_auth", post(CreateAuthRequest::create_new_auth))
    .route("/find_auths", get(FindAuthRequest::find_auths))
}