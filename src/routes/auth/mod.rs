pub mod user_login;
pub mod auth_model;
pub mod create_auth;
pub mod find_auths;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    user_login::UserLoginRequest,
    create_auth::CreateAuthRequest,
    find_auths::FindAuthRequest,
};

pub async fn create_auth_routes() -> Router {
    Router::new()
    .route("/login", post(UserLoginRequest::user_login))
    .route("/create_new_auth", post(CreateAuthRequest::create_new_auth))
    .route("/find_auths", get(FindAuthRequest::find_auths))
}