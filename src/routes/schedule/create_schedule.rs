use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate, Month };
use axum::{Extension, Json, extract::{Path}, http};
use serde_json::{Value, json};
use http_serde;

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleRequest {
    pub schedule_month: String,
    pub schedule_year: String,
    pub drawing_no: String,
    pub similar_part_no: Option<String>,
    pub customer_plant: String,
    pub supplier_plant: String,
    pub most_critical_qty: Option<i64>,
    pub most_critical_commitment_date: Option<String>,
    pub critical_qty: Option<i64>,
    pub critical_commitment_date: Option<String>,
    pub mis_qty: Option<i64>,
    pub mis_commitment_date: Option<String>,
    pub recv_till: Option<i64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateScheduleRequest {
    pub async fn create_schedule_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_schedule_table (
                id SERIAL NOT NULL,
                schedule_pk TEXT NOT NULL,
                schedule_month TEXT NOT NULL,
                schedule_year TEXT NOT NULL,
                drawing_no TEXT NOT NULL,
                similar_part_no TEXT NOT NULL,
                customer_plant TEXT NOT NULL,
                supplier_plant TEXT NOT NULL,
                most_critical_qty BIGINT,
                most_critical_commitment_date DATE,
                critical_qty BIGINT,
                critical_commitment_date DATE,
                mis_qty BIGINT,
                mis_commitment_date DATE,
                recv_till BIGINT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (schedule_month, schedule_year, drawing_no)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_schedule_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_schedule_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn truncate_schedule_table(
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
        
        match service.client
        .execute(
            "TRUNCATE TABLE mwspl_schedule_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_schedule(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
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

        match service.client
        .execute(
            "INSERT INTO mwspl_schedule_table (
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
                recv_till,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.schedule_month,
                &payload.schedule_year,
                &payload.drawing_no,
                &payload.similar_part_no,
                &payload.customer_plant,
                &payload.supplier_plant,
                &payload.most_critical_qty,
                &payload.most_critical_commitment_date,
                &payload.critical_qty,
                &payload.critical_commitment_date,
                &payload.mis_qty,
                &payload.mis_commitment_date,
                &payload.recv_till,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn upload_schedule_csv(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        let mut rdr = csv::Reader::from_path("D:/rust_projects/intellidyn/july_schedule.csv").unwrap();
        let schedule_vector: Vec<CreateScheduleRequest> = Vec::new();
        let mut counter = 0;

        for result in rdr.records() {
            let record = result.unwrap();
            let schedule: CreateScheduleRequest = record.deserialize(None).unwrap();

            let most_critical_commit_date = match &schedule.most_critical_commitment_date {
                Some(v) => Some(NaiveDate::parse_from_str(&v, "%m/%d/%Y").expect("Parsing error")),
                None => None
            };

            let critical_commit_date = match &schedule.critical_commitment_date {
                Some(v) => Some(NaiveDate::parse_from_str(&v, "%m/%d/%Y").expect("Parsing error")),
                None => None
            };

            let mis_commit_date = match &schedule.mis_commitment_date {
                Some(v) => Some(NaiveDate::parse_from_str(&v, "%m/%d/%Y").expect("Parsing error")),
                None => None
            };

            service.client
            .execute(
                "INSERT INTO mwspl_schedule_table (
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
                    recv_till,
                    created_by,
                    created_on,
                    created_login_key,
                    modified_by,
                    modified_on,
                    modified_login_key
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
                &[
                    &Uuid::new_v4().to_string(),
                    &schedule.schedule_month,
                    &schedule.schedule_year,
                    &schedule.drawing_no,
                    &schedule.similar_part_no,
                    &schedule.customer_plant,
                    &schedule.supplier_plant,
                    &schedule.most_critical_qty,
                    &most_critical_commit_date,
                    &schedule.critical_qty,
                    &critical_commit_date,
                    &schedule.mis_qty,
                    &mis_commit_date,
                    &schedule.recv_till,
                    &user,
                    &Local::now(),
                    &login_key,
                    &None::<String>,
                    &None::<DateTime<Local>>,
                    &None::<String>
                ]
            )
            .await
            .map(|val| {counter = counter + 1})
            .map_err(|err| println!("{}",err.to_string()));
        }

        Json(json!(CreateScheduleResponse {
            status_code: http::StatusCode::OK,
            data: Some(format!("{} data entries successful", counter)),
            error: None
        }))
    }
}