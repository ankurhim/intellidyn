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

use crate::routes::approved_components::ApprovedComponent;
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApprovedComponentRequest {
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
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
            "CREATE TABLE IF NOT EXISTS mwspl_approved_component_table (
                id SERIAL NOT NULL,
                approval_pk TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                grade TEXT NOT NULL,
                section INT NOT NULL,
                section_type TEXT NOT NULL,
                approved_part TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (heat_no, grade, section, section_type, approved_part),
                CONSTRAINT pk_approval PRIMARY KEY (heat_no, grade, section, section_type, approved_part)
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
       
                service.client
                .execute(
                    "INSERT INTO mwspl_approved_component_table (
                        approval_pk,
                        heat_no,
                        grade,
                        section,
                        section_type,
                        approved_part,
                        created_by,
                        created_on,
                        created_login_key,
                        modified_by,
                        modified_on,
                        modified_login_key,
                        remarks
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
                    &[
                        &Uuid::new_v4().to_string(),
                        &payload.heat_no.clone(),
                        &payload.grade.clone(),
                        &payload.section.clone(),
                        &payload.section_type.clone(),
                        &part.to_string(),
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
                .map(|val| i += val)
                .map_err(|e| Json(json!(e.to_string())));
            }
        };
        Json(json!(i))
    }
}