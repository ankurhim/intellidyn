pub mod user_login;

use axum::{
    Router,
    routing::{post},
};

use self::{
    user_login::UserLoginRequest,
};

pub async fn create_auth_routes() -> Router {
    Router::new()
    .route("/login", post(UserLoginRequest::user_login))
}