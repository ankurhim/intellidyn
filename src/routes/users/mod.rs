pub mod user_model;
pub mod create_user;
pub mod find_users;
pub mod user_login;
pub mod user_logout;
pub mod update_user;
// pub mod delete_user;
// pub mod update_user;

use axum::{
    Router,
    routing::{get, post, put},
};

use self::{
    create_user::CreateUserRequest,
    find_users::FindUserRequest,
    user_login::UserLoginRequest,
    user_logout::UserLogoutRequest,
    update_user::UpdateUserRequest,
    // delete_user::DeleteUserRequest,
    // update_user::UpdateUserRequest,
};

pub async fn create_user_routes() -> Router {
    Router::new()
    .route("/create_user_table", post(CreateUserRequest::create_user_table))
    .route("/drop_user_table", post(CreateUserRequest::drop_user_table))
    .route("/:username/:login_key/create_new_user", post(CreateUserRequest::create_new_user))
    .route("/:username/:login_key/find_users", get(FindUserRequest::find_users))
    .route("/login", post(UserLoginRequest::user_login))
    .route("/:username/:login_key/logout", get(UserLogoutRequest::user_logout))
    .route("/:username/:login_key/change_password", put(UpdateUserRequest::change_password))
    // .route("/find_user_by_username", get(FindUserRequest::find_user_by_username))
    // .route("/delete_user_by_username", delete(DeleteUserRequest::delete_user_by_username))
    // .route("/update_user_by_username", put(UpdateUserRequest::update_user_by_username))
}