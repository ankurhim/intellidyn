use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIncomingSteelRequest {
    pub challan_no: String,
    pub challan_date: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub jominy_value: Option<String>,
    pub opening_qty: i64,
    pub received_qty: i64,
    pub remarks: Option<String>
}

impl CreateIncomingSteelRequest {
    pub async fn create_incoming_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_incoming_steel_table(
                id SERIAL NOT NULL,
                incoming_steel_pk TEXT NOT NULL,
                challan_no TEXT NOT NULL PRIMARY KEY,
                challan_date DATE NOT NULL,
                grade TEXT NOT NULL,
                section BIGINT NOT NULL,
                section_type TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                heat_code TEXT,
                jominy_value TEXT,
                opening_qty BIGINT NOT NULL,
                received_qty BIGINT NOT NULL,
                issued_qty BIGINT NOT NULL,
                actual_qty BIGINT NOT NULL,
                heat_status TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (challan_no, grade, section, section_type, heat_no)
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

    pub async fn drop_incoming_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let drop_incoming_steel_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_incoming_steel_table;",
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

    pub async fn create_new_incoming_steel(
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

        let challan_date = NaiveDate::parse_from_str(&payload.challan_date, "%d-%m-%Y").expect("Challan Date parsing error");
        
        match service.client
        .execute(
            "INSERT INTO mwspl_incoming_steel_table(
                incoming_steel_pk,
                challan_no,
                challan_date,
                grade,
                section,
                section_type,
                heat_no,
                heat_code,
                jominy_value,
                opening_qty,
                received_qty,
                issued_qty,
                actual_qty,
                heat_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,$17, $18, $19, $20, $21)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.challan_no,
                &challan_date,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.heat_no,
                &payload.heat_code,
                &payload.jominy_value,
                &payload.opening_qty,
                &payload.received_qty,
                &0_i64,
                &(payload.opening_qty + payload.received_qty),
                &None::<String>,
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