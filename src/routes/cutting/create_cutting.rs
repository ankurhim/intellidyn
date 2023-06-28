use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}, http::StatusCode};
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::requisition::requisition_model::Requisition;

pub struct CreateCuttingRequest {
    pub cutting_pk: String,
    pub requisition_id: String,
    pub planned_date: NaiveDate,
    pub machine: String,
    pub part_code: String,
    pub steel_code: String,
    pub heat_no: String,
    pub planned_qty: i64
}

impl CreateCuttingRequest {
    pub async fn create_cutting_temp_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        let create_table = service.client
        .batch_execute(
            "SET experimental_enable_temp_tables = 'on';CREATE TEMP TABLE mwspl_cutting_temp(
                temp_id SERIAL NOT NULL PRIMARY KEY,
                cutting_pk TEXT NOT NULL UNIQUE,
                requisition_id TEXT NOT NULL,
                planned_date DATE NOT NULL,
                machine TEXT NOT NULL,
                part_code TEXT NOT NULL,
                steel_code TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                planned_qty BIGINT NOT NULL
            );"
        )
        .await;

        Json(json!("Completed"))
    }
    pub async fn insert_cutting_temp_table(
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<CreateCuttingRequest>
    ) -> Json<Value> {
        let insert_values = service.clone().client
        .execute(
            "INSERT INTO mwspl_cutting_temp(
                cutting_pk,
                requisition_id,
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty,
            ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.requisition_id,
                &payload.planned_date,
                &payload.machine,
                &payload.part_code,
                &payload.steel_code,
                &payload.heat_no,
                &payload.planned_qty
            ]
        )
        .await;

        println!("{:?}", insert_values);

        Json(json!("Completed"))
    }
}