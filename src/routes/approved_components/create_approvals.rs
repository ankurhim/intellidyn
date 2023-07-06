use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{Local, DateTime};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path
};

use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApprovedComponentRequest {
    pub heat_no: String,
    pub part_list: Vec<String>,
    pub remarks: Option<String>
}

#[derive(Debug, Serialize)]
pub struct CreateApprovedComponentResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateApprovedComponentRequest {
    pub async fn create_approved_components_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_approved_component_table(
                id SERIAL NOT NULL,
                approval_pk TEXT NOT NULL PRIMARY KEY,
                rm_id TEXT NOT NULL REFERENCES mwspl_incoming_steel_table(incoming_steel_pk) ON UPDATE CASCADE ON DELETE NO ACTION,
                heat_no TEXT NOT NULL,
                approved_part TEXT NOT NULL REFERENCES mwspl_part_table(part_code) ON UPDATE CASCADE ON DELETE NO ACTION,
                avail_qty FLOAT8 NOT NULL,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (heat_no, approved_part)
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

    pub async fn drop_approved_components_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let drop_incoming_steel_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_approved_component_table;",
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

    pub async fn create_new_approved_components(
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

        let mut i: u64 = 0;
        if &payload.part_list.len() > &0 {
            for part in &payload.part_list {

                match service.client
                .execute(
                    "CREATE TABLE IF NOT EXISTS temp_approvals (
                        approval_pk TEXT NOT NULL PRIMARY KEY,
                        heat_no TEXT NOT NULL,
                        approved_part TEXT NOT NULL
                    )",
                    &[]
                )
                .await
                .map(|v| v)
                .map_err(|e| e.to_string()) {
                    Ok(v) => v,
                    Err(e) => 0
                };

                match service.client
                .execute(
                    "INSERT INTO temp_approvals(heat_no, approved_part)",
                    &[
                        &payload.heat_no,
                        &part.to_string(),
                    ]
                )
                .await
                .map(|v| v)
                .map_err(|e| e.to_string()) {
                    Ok(v) => v,
                    Err(e) => 0
                };
       
                match service.client
                .execute(
                    "INSERT INTO mwspl_approved_component_table (
                        approval_pk,
                        rm_id,
                        heat_no,
                        approved_part,
                        avail_qty,
                        created_by,
                        created_on,
                        created_login_key,
                        modified_by,
                        modified_on,
                        modified_login_key
                    ) VALUES (
                        $1,
                        (SELECT incoming_pk FROM mwspl_incoming_steel_table WHERE heat_no = $2),
                        $2,
                        $3,
                        (SELECT received_qty FROM mwspl_incoming_steel_table WHERE heat_no = $2),
                        $4,
                        $5,
                        $6,
                        $7,
                        $8,
                        $9
                    )",
                    &[
                        &Uuid::new_v4().to_string(),
                        &payload.heat_no.clone(),
                        &part.to_string(),
                        &user,
                        &Local::now(),
                        &login_key,
                        &None::<String>,
                        &None::<DateTime<Local>>,
                        &None::<String>
                    ]
                )
                .await
                .map(|val| i += val)
                .map_err(|e| Json(json!(e.to_string()))) {
                    Ok(_) => (),
                    Err(_) => ()
                };
            }
        };
        
        match service.client
        .execute("DROP TABLE IF EXISTS temp_approvals;", &[]).await
        .map(|v| Json(json!(v.to_string())))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => Json(json!(i)),
            Err(e) => e
        }
    }
}