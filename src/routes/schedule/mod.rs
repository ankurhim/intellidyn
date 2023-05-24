pub mod schedule_model;
pub mod create_schedule;
pub mod find_schedule;
pub mod find_schedule_part_list;

use axum::{
    Router,
    routing::{get, post}
};

use self::{
    create_schedule::CreateScheduleRequest,
    find_schedule::FindScheduleRequest,
    find_schedule_part_list::FindSchedulePartListRequest,
};

pub async fn create_schedule_routes() -> Router {
    Router::new()
    .route("/create_schedule_table", post(CreateScheduleRequest::create_schedule_table))
    .route("/drop_schedule_table", post(CreateScheduleRequest::drop_schedule_table))
    .route("/:user/:login_key/truncate_schedule_table", post(CreateScheduleRequest::truncate_schedule_table))
    .route("/:user/:login_key/create_new_schedule", post(CreateScheduleRequest::create_new_schedule))
    .route("/:user/:login_key/find_schedule", get(FindScheduleRequest::find_schedule))
    .route("/:user/:login_key/find_schedule_part_list", get(FindSchedulePartListRequest::find_schedule_part_list))
}