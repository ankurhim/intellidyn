use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{Local, DateTime, NaiveDate};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path
};
use std::collections::HashMap;

use serde_json::{Value, json};

use crate::routes::cutting_production::cutting_request_model::SteelRequisition;
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelRequisitionRequest {
    pub planned_date: String,
    pub machine_no: String,
    pub part_no: String,
    pub heat_no: String,
    pub planned_quantity: i64,
    pub remarks: Option<String>
}

impl CreateSteelRequisitionRequest {
    pub async fn create_steel_requisition_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_steel_requisition_table (
                id SERIAL NOT NULL,
                request_pk TEXT NOT NULL,
                planned_date DATE NOT NULL,
                machine_no TEXT NOT NULL,
                part_no TEXT NOT NULL,
                heat_no TEXT NOT NULL REFERENCES mwspl_incoming_steel_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                planned_quantity BIGINT NOT NULL,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (heat_no, part_no, machine_no)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_steel_requisition_table(
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let drop_incoming_steel_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_steel_requisition_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match drop_incoming_steel_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_steel_requisition(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
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

        let planned_date = NaiveDate::parse_from_str(&payload.planned_date, "%d-%m-%Y").expect("Challan Date parsing error");
        
        match service.client
        .execute(
            "INSERT INTO mwspl_steel_requisition_table(
                request_pk,
                planned_date,
                machine_no,
                part_no,
                heat_no,
                planned_quantity,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key,
                remarks TEXT
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
            &[
                &Uuid::new_v4().to_string(),
                &planned_date,
                &payload.machine_no,
                &payload.part_no,
                &payload.heat_no,
                &payload.planned_quantity,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>,
                &payload.remarks
            ]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}