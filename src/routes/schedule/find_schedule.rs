use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
    extract::Path
};
use tokio_postgres::Row;
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::schedule::schedule_model::Schedule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindScheduleRequest {
    pub drawing_no: String
}

impl FindScheduleRequest {
    pub async fn find_schedule(
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

        let resp = service.client
        .query(
            "SELECT
            id,
            schedule_pk,
            schedule_month,
            schedule_year,
            drawing_no,
            similar_part_no,
            customer_plant,
            supplier_plant,
            most_critical_qty,
            most_critical_commitment_date,
            critical_qty,
            critical_commitment_date,
            mis_qty,
            mis_commitment_date,
            most_critical_qty + critical_qty + mis_qty AS total_forging_qty,
            recv_till,
            most_critical_qty + critical_qty + mis_qty - recv_till AS balance_qty,
            created_by,
            created_on,
            created_login_key,
            modified_by,
            modified_on,
            modified_login_key
            FROM mwspl_schedule_table;",
            &[]
        )
        .await
        .map_err(|err| Json(json!(err.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Schedule> = Vec::new();
    
    for row in row_vector {
        vector.push(Schedule {
            schedule_pk: Uuid::parse_str(row.get(1)).unwrap(),
            schedule_month: row.get(2),
            schedule_year: row.get(3),
            drawing_no: row.get(4),
            similar_part_no: row.get(5),
            customer_plant: row.get(6),
            supplier_plant: row.get(7),
            most_critical_qty: row.get(8),
            most_critical_commitment_date: row.get(9),
            critical_qty: row.get(10),
            critical_commitment_date: row.get(11),
            mis_qty: row.get(12),
            mis_commitment_date: row.get(13),
            total_forging_qty: row.get(14),
            recv_till: row.get(15),
            balance_qty: row.get(16),
            created_by: row.get(17),
            created_on: row.get(18),
            created_login_key: row.get(19),
            modified_by: row.get(20),
            modified_on: row.get(21),
            modified_login_key: row.get(22)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Schedule>>)),
        _ => Json(json!(vector))
    }
}