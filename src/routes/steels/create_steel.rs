use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelRequest {
    pub steel_code: String,
    pub steel_grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub remarks: Option<String>
}

impl CreateSteelRequest {
    pub async fn create_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.execute("CREATE TABLE IF NOT EXISTS mwspl_steel_table(
            id SERIAL NOT NULL,
            steel_code TEXT NOT NULL PRIMARY KEY,
            steel_grade TEXT NOT NULL,
            section BIGINT NOT NULL,
            section_type TEXT NOT NULL,
            jominy_range TEXT,
            created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
            created_on TIMESTAMPTZ NOT NULL,
            created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
            modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
            modified_on TIMESTAMPTZ,
            modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
            remarks TEXT,
            UNIQUE (steel_code)
        );", &[])
        .await
        .map(val => Json(json!(val)))
        .map_err(e => Json(json!(err))) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}