use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}, http::StatusCode};
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::requisition::requisition_model::Requisition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCuttingRequest {
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
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Json<Value> {
        let create_temp_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_cutting_temp(
                temp_id SERIAL NOT NULL PRIMARY KEY,
                cutting_pk TEXT NOT NULL UNIQUE,
                requisition_id TEXT NOT NULL,
                planned_date DATE NOT NULL,
                machine TEXT NOT NULL,
                part_code TEXT NOT NULL,
                steel_code TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                planned_qty BIGINT NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT
            );",
            &[]
        )
        .await;

        println!("{:?}", create_temp_table);

        let insert_values = service.client
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
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.requisition_id,
                &payload.planned_date,
                &payload.machine,
                &payload.part_code,
                &payload.steel_code,
                &payload.heat_no,
                &payload.planned_qty,
                &"admin",
                &Local::now(),
                &"test",
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>,
            ]
        )
        .await;

        println!("{:?}", insert_values);

        let create_cutting_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_cutting_table(
                id SERIAL NOT NULL PRIMARY KEY,
                requisition_id TEXT NOT NULL,
                rm_id TEXT NOT NULL,
                cutting_pk TEXT NOT NULL UNIQUE,
                planned_date DATE NOT NULL,
                machine TEXT NOT NULL,
                part_no TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                heat_code TEXT,
                grade TEXT NOT NULL,
                section TEXT NOT NULL,
                section_type TEXT NOT NULL,
                cut_wt FLOAT8 NOT NULL,
                planned_qty BIGINT NOT NULL,
                actual_qty BIGINT DEFAULT 0,
                ok_qty BIGINT DEFAULT 0,
                rej_qty BIGINT DEFAULT 0,
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
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION
            )",
            &[]
        )
        .await;

        println!("{:?}", create_cutting_table);

        service.client
        .execute(
            "UPDATE mwspl_requisition_table
            SET request_status = 'CLOSED'
            WHERE requisition_id = $1", &[&payload.requisition_id]
        )
        .await;

        let insert = service.client
        .execute(
            "INSERT INTO mwspl_cutting_table(
                cutting_pk,
                requisition_id,
                rm_id,
                planned_date,
                machine,
                part_no,
                heat_no,
                heat_code,
                grade,
                section,
                section_type,
                cut_wt,
                planned_qty,
                rej_qty,
                ok_wt,
                rej_wt,
                total_wt,
                issued_qty,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) SELECT c.cutting_pk,
                c.requisition_id,
                a.rm_id,
                c.planned_date,
                c.machine,
                p.part_no,
                c.heat_no,
                i.heat_code,
                s.steel_grade,
                s.section,
                s.section_type,
                p.cut_weight,
                c.planned_qty,
                (SELECT (actual_qty - ok_qty) FROM mwspl_cutting_table),
                (p.cut_weight * (SELECT ok_qty FROM mwspl_cutting_table)::FLOAT8),
                (p.cut_weight * ((SELECT rej_qty FROM mwspl_cutting_table)::FLOAT8)),
                (p.cut_weight * ((SELECT actual_qty FROM mwspl_cutting_table)::FLOAT8)),
                (SELECT ok_qty FROM mwspl_cutting_table),
                c.created_by,
                c.created_on,
                c.created_login_key,
                c.modified_by,
                c.modified_on,
                c.modified_login_key
                FROM mwspl_cutting_temp c
                INNER JOIN mwspl_part_table p
                ON p.part_code = c.part_code
                INNER JOIN mwspl_approved_component_table a
                ON a.heat_no = c.heat_no
                AND a.approved_part = (SELECT part_no FROM mwspl_part_table WHERE part_code = c.part_code)
                AND a.avail_qty >= ((c.planned_qty::FLOAT8) * p.cut_weight)
                INNER JOIN mwspl_steel_table s
                ON c.steel_code = s.steel_code
                INNER JOIN mwspl_incoming_steel_table i
                ON a.rm_id = i.incoming_steel_pk;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        println!("{:?}", insert);

        service.client
        .execute("DROP TABLE IF EXISTS mwspl_cutting_temp;", &[])
        .await;



        match insert {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_cutting_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let drop_cutting_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_cutting_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match drop_cutting_table {
            Ok(v) => v,
            Err(e) => e
        }
    }
}