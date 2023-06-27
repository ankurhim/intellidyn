use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}, http};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCuttingRequest {
    pub requisition_id: String,
    pub cutting_pk: Uuid,
    pub planned_date: NaiveDate,
    pub machine: String,
    pub part_code: String,
    pub steel_code: String,
    pub heat_no: String,
    pub planned_qty: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCuttingResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateCuttingRequest {
    pub async fn create_cutting_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_cutting_table (
                id SERIAL NOT NULL,
                cutting_pk TEXT NOT NULL,
                requisition_id TEXT NOT NULL,
                rm_id TEXT NOT NULL,
                planned_date DATE NOT NULL,
                machine TEXT NOT NULL,
                part_no TEXT NOT NULL REFERENCES mwspl_part_table(part_code) ON UPDATE CASCADE ON DELETE NO ACTION,
                heat_no TEXT NOT NULL REFERENCES mwspl_incoming_steel_table(heat_no) ON UPDATE CASCADE ON DELETE NO ACTION,
                steel_code TEXT NULL REFERENCES mwspl_steel_table(steel_code) ON UPDATE CASCADE ON DELETE NO ACTION,
                cut_wt FLOAT8 NOT NULL REFERENCES mwspl_part_table(cut_weight) ON UPDATE CASCADE ON DELETE NO ACTION, 
                planned_qty BIGINT NOT NULL,
                actual_qty BIGINT,
                ok_qty BIGINT,
                rej_qty BIGINT,
                ok_wt FLOAT8,
                rej_wt FLOAT8,
                end_pc_wt FLOAT8,
                total_wt FLOAT8,
                issued_qty BIGINT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (cutting_pk)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateCuttingResponse{
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|e| Json(json!(CreateCuttingResponse{
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(e.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_cutting_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_cutting_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateCuttingResponse{
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|e| Json(json!(CreateCuttingResponse{
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(e.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    // pub async fn create_new_cutting(
    //     Path((user, login_key)): Path<(String, String)>,
    //     Extension(service): Extension<Arc<DbService>>,
    //     Json(payload): Json<Self>,
    // ) -> Json<Value> {

    //     let resp = service.client
    //     .query(
    //         "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
    //     )
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     for row in resp.unwrap() {
    //         if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
    //             break;
    //         } else {
    //             return Json(json!("You are logged out"));
    //         }
    //     }

    //     let temp_cutting_table = service.client.execute(
    //         "CREATE TEMP TABLE mwspl_temp_cutting_table(
    //             temp_id SERIAL NOT NULL,
    //             cutting_pk TEXT NOT NULL,
    //             requisition_id TEXT NOT NULL,
    //             planned_date DATE NOT NULL,
    //             machine TEXT NOT NULL REFERENCES mwspl_machine_table(machine_id) ON UPDATE CASCADE ON DELETE NO ACTION,
    //             part_no TEXT NOT NULL REFERENCES mwspl_part_table(part_code) ON UPDATE CASCADE ON DELETE NO ACTION,
    //             heat_no TEXT NOT NULL REFERENCES mwspl_incoming_steel_table(heat_no) ON UPDATE CASCADE ON DELETE NO ACTION,
    //             steel_code TEXT NULL  REFERENCES mwspl_steel_table(steel_code) ON UPDATE CASCADE ON DELETE NO ACTION,
    //             planned_qty BIGINT NOT NULL
    //         )",
    //         &[]
    //     )
    //     .await
    //     .map( |val| val)
    //     .map_err(|e| e);

    //     let insert = match temp_cutting_table {
    //         Ok(_) => service.client
    //         .execute(
    //             "INSERT INTO mwspl_temp_cutting_table(
    //                 cutting_pk,
    //                 requisition_id,
    //                 planned_date,
    //                 machine,
    //                 part_no,
    //                 heat_no,
    //                 steel_code,
    //                 planned_qty
    //             ) VALUE ($1, $2, $3, $4, $5, $6, $7, $8)",
    //             &[
    //                 &Uuid::new_v4().to_string(),
    //                 &payload.requisition_id,
    //                 &payload.planned_date,
    //                 &payload.machine,
    //                 &payload.part_code,
    //                 &payload.heat_no,
    //                 &payload.steel_code,
    //                 &payload.planned_qty
    //             ]
    //         )
    //         .await
    //         .map( |val| val)
    //         .map_err(|e| e),
    //         Err(e) => Json(json!(CreateCuttingResponse {
    //             status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
    //             data: None,
    //             error: Some(e.to_string())
    //         }));

    //         match insert {
    //             Ok(_) => service.client
    //             .execute(
    //                 "INSERT INTO mwspl_cutting_table (requisition_id, rm_id, cutting_id, planned_date, machine, part_no, heat_no, steel_code, planned_qty)
    //                 SELECT
    //                 c.requisition_id,
    //                 a.rm_id,
    //                 c.cutting_id,
    //                 c.planned_date,
    //                 c.machine,
    //                 p.part_no,
    //                 c.heat_no,
    //                 g.heat_code,
    //                 s.grade,
    //                 s.size,
    //                 s.section,
    //                 p.cut_wt,
    //                 c.planned_qty
    //                 FROM cutting_temp c
    //                 INNER JOIN part p
    //                 ON p.part_code = c.part_code
    //                 INNER JOIN approved_components a
    //                 ON a.heat_no = c.heat_no
    //                 AND a.part_no = (SELECT part_no FROM mwspl_part_table WHERE part_code = c.part_code)
    //                 AND a.avail_qty >= (planned_qty * p.cut_wt)
    //                 INNER JOIN steels s
    //                 ON c.steel_code = s.steel_code
    //                 INNER JOIN gate_entry g
    //                 ON a.rm_id = g.gate_entry_id;",
    //                 &[]
    //             )
    //             .await
    //             .map(|val| Json(json!(CreateCuttingResponse{
    //                 status_code: http::StatusCode::OK,
    //                 data: Some(val.to_string()),
    //                 error: None
    //             })))
    //             .map_err(|e| Json(json!(CreateCuttingResponse{
    //                 status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
    //                 data: None,
    //                 error: Some(e.to_string())
    //             }))) {
    //                 Ok(v) => v,
    //                 Err(e) => e
    //             },
    //             Err(e) => Json(json!(CreateCuttingResponse {
    //                 status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
    //                 data: None,
    //                 error: Some(e.to_string())
    //             })) 
    //         }
    //     };
    //     insert
    // }
}