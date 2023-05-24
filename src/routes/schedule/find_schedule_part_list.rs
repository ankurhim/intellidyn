use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, Datelike,Month };
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use tokio_postgres::Row;
use serde_json::{Value, json};
use num_traits::FromPrimitive;

use crate::service::DbService;
use crate::routes::requisition::requisition_model::Requisition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSchedulePartListRequest;

impl FindSchedulePartListRequest {
    pub async fn find_schedule_part_list(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut part_list: Vec<String> = Vec::new();

        let rows = service.client
        .query(
            "SELECT drawing_no FROM mwspl_schedule_table WHERE schedule_month = $1 AND schedule_year = $2;",
            &[
                &Month::from_u32(Local::now().month()).unwrap().name(),
                &Local::now().year().to_string()
            ]
        )
        .await
        .map_err(|err| Json(json!(err.to_string())));

        for row in rows.unwrap() {

            if row.len() != 0 {
                part_list.push(row.get::<usize, String>(0).to_string())
            } else {
                return Json(json!("Table is empty"));
            }
        }

        Json(json!(part_list))
    }
}