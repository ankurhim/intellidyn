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
    pub steel_code: String,
    pub heat_no: String,
    pub heat_code: Option<String>,
    pub jominy_value: Option<String>,
    pub received_qty: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIncomingSteelResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateIncomingSteelRequest {
    pub async fn create_incoming_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_incoming_steel_table(
                id SERIAL NOT NULL,
                incoming_steel_pk TEXT NOT NULL PRIMARY KEY,
                challan_no TEXT NOT NULL,
                challan_date DATE NOT NULL,
                steel_code TEXT NOT NULL REFERENCES mwspl_steel_table(steel_code) ON UPDATE CASCADE ON DELETE NO ACTION,
                heat_no TEXT NOT NULL,
                heat_code TEXT,
                jominy_value TEXT,
                received_qty BIGINT NOT NULL,
                heat_status TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (challan_no, heat_no, incoming_steel_pk)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateIncomingSteelResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateIncomingSteelResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_incoming_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_incoming_steel_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(CreateIncomingSteelResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateIncomingSteelResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
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

        let challan_date = NaiveDate::parse_from_str(&payload.challan_date, "%Y-%m-%d").expect("Challan Date parsing error");
        
        match service.client
        .execute(
            "INSERT INTO mwspl_incoming_steel_table(
                incoming_steel_pk,
                challan_no,
                challan_date,
                steel_code,
                heat_no,
                heat_code,
                jominy_value,
                received_qty,
                heat_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.challan_no,
                &challan_date,
                &payload.steel_code,
                &payload.heat_no,
                &payload.heat_code,
                &payload.jominy_value,
                &payload.received_qty,
                &None::<String>,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>
            ]
        )
        .await
        .map(|val| Json(json!(CreateIncomingSteelResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateIncomingSteelResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }
}