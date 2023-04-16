pub mod user_model;
pub mod create_user;
pub mod find_users;

use axum::{
    Router,
    routing::{get, post},
    response::Json,
};

use serde_json::{Value, json};
use self::{
    user_model::User,
    create_user::CreateUserRequest,
    find_users::FindUserRequest,
};

pub async fn create_user_routes() -> Router {
    Router::new()
    .route("/create_new_user", post(CreateUserRequest::create_new_user))
    .route("/find_users", get(FindUserRequest::find_users))
    .route("/find_user_by_id", get(FindUserRequest::find_user_by_id))
    .route("/user_login", post(FindUserRequest::user_login))
}