use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Path}, http};
use serde_json::{Value, json};
use http_serde;

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelRequest {
    pub steel_code: String,
    pub steel_grade: String,
    pub is_nickel_steel: bool,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateSteelRequest {
    pub async fn create_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        
        match service.client
        .execute("CREATE TABLE IF NOT EXISTS mwspl_steel_table(
            id SERIAL NOT NULL,
            steel_pk TEXT NOT NULL,
            steel_code TEXT NOT NULL PRIMARY KEY,
            steel_grade TEXT NOT NULL,
            is_nickel_steel BOOLEAN NOT NULL,
            section BIGINT NOT NULL,
            section_type TEXT NOT NULL,
            jominy_range TEXT,
            steel_status TEXT,
            created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
            created_on TIMESTAMPTZ NOT NULL,
            created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
            modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
            modified_on TIMESTAMPTZ,
            modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
            UNIQUE (steel_code)
        );", &[])
        .await
        .map(|val| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_steel_table(  
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_steel_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_steel(
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
                return Json(json!(CreateSteelResponse {
                    status_code: http::StatusCode::UNAUTHORIZED,
                    data: None,
                    error: Some("Unauthorized Access".to_string())
                }));
            }
        }

        match service.client
        .execute(
           "INSERT INTO mwspl_steel_table(
            steel_pk,
            steel_code,
            steel_grade,
            is_nickel_steel,
            section,
            section_type,
            jominy_range,
            steel_status,
            created_by,
            created_on,
            created_login_key,
            modified_by,
            modified_on,
            modified_login_key
           ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",&[
            &Uuid::new_v4().to_string(),
                &payload.steel_code,
                &payload.steel_grade,
                &payload.is_nickel_steel,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
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
        .map(|val| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn upload_steel_csv(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        let mut rdr = csv::Reader::from_path("F:/rust_projects/intellidyn/steel.csv").unwrap();
        let steel_vector: Vec<CreateSteelRequest> = Vec::new();
        let mut counter = 0;

        for result in rdr.records() {
            let record = result.unwrap();
            let steel: CreateSteelRequest = record.deserialize(None).unwrap();

            match service.client
            .execute(
            "INSERT INTO mwspl_steel_table(
                steel_pk,
                steel_code,
                steel_grade,
                is_nickel_steel,
                section,
                section_type,
                jominy_range,
                steel_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",&[
                &Uuid::new_v4().to_string(),
                    &steel.steel_code,
                    &steel.steel_grade,
                    &steel.is_nickel_steel,
                    &steel.section,
                    &steel.section_type,
                    &steel.jominy_range,
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
            .map(|val| {counter = counter + 1})
            .map_err(|e| ()) {
                Ok(v) => (),
                Err(e) => ()
            };
        }

        Json(json!(CreateSteelResponse {
            status_code: http::StatusCode::OK,
            data: Some(format!("{} data entries successful", counter)),
            error: None
        }))
    }
}