pub mod machine_model;
pub mod create_machine;
pub mod find_machine;
// pub mod update_machine;

use axum::{
    Router,
    routing::{post, get},
};

use self::{
    create_machine::CreateMachineRequest,
    find_machine::FindMachineRequest,
};

pub async fn create_machine_routes() -> Router {
    Router::new()
    .route("/create_machine_table", post(CreateMachineRequest::create_machine_table))
    .route("/drop_machine_table", post(CreateMachineRequest::drop_machine_table))
    .route("/:user/:login_key/create_new_machine", post(CreateMachineRequest::create_new_machine))
    .route("/:user/:login_key/find_all_machines", get(FindMachineRequest::find_all_machines))
}