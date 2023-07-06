use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Path}, http};
use serde_json::{Value, json};
use http_serde;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartRequest {
    pub part_code: Option<String>,
    pub part_no: String,
    pub part_name: String,
    pub dwg_rev_no: Option<String>,
    pub steel_code: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreatePartRequest {
    pub async fn create_part_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        
        match service.client
        .execute("CREATE TABLE IF NOT EXISTS mwspl_part_table(
            id SERIAL NOT NULL,
            part_pk TEXT NOT NULL,
            part_code TEXT,
            part_no TEXT NOT NULL PRIMARY KEY,
            part_name TEXT NOT NULL,
            dwg_rev_no TEXT,
            part_status TEXT,
            steel_code TEXT REFERENCES mwspl_steel_table(steel_code) ON UPDATE NO ACTION ON DELETE NO ACTION,
            created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
            created_on TIMESTAMPTZ NOT NULL,
            created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
            modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
            modified_on TIMESTAMPTZ,
            modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
            UNIQUE INDEX (part_no, part_code)
        );", &[])
        .await
        .map(|val| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_part_table( 
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_part_table;", &[]
        )
        .await
        .map(|val| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_part(
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
                return Json(json!(CreatePartResponse {
                    status_code: http::StatusCode::UNAUTHORIZED,
                    data: None,
                    error: Some("The user is not authorized".to_string())
                }));
            }
        }

        match service.client
        .execute(
           "INSERT INTO mwspl_part_table(
            part_pk,
            part_code,
            part_no,
            part_name,
            dwg_rev_no,
            steel_code,
            part_status,
            created_by,
            created_on,
            created_login_key,
            modified_by,
            modified_on,
            modified_login_key
           ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
           &[
            &Uuid::new_v4().to_string(),
            &payload.part_code,
            &payload.part_no,
            &payload.part_name,
            &payload.dwg_rev_no,
            &payload.steel_code,
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
        .map(|val| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::OK,
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartResponse {
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn upload_part_csv(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        let mut rdr = csv::Reader::from_path("F:/rust_projects/intellidyn/part.csv").unwrap();
        let part_vector: Vec<CreatePartRequest> = Vec::new();
        let mut counter = 0;

        for result in rdr.records() {
            let record = result.unwrap();
            let part: CreatePartRequest = record.deserialize(None).unwrap();

            println!("{:?}", &part);

            let result = service.client
            .execute(
                "INSERT INTO mwspl_part_table(
                    part_pk,
                    part_code,
                    part_no,
                    part_name,
                    dwg_rev_no,
                    steel_code,
                    part_status,
                    created_by,
                    created_on,
                    created_login_key,
                    modified_by,
                    modified_on,
                    modified_login_key
                   ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",&[
                    &Uuid::new_v4().to_string(),
                    &part.part_code,
                    &part.part_no,
                    &part.part_name,
                    &part.dwg_rev_no,
                    &part.steel_code,
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
            .map(|val| {counter = counter + 1});

            println!("{:?}", &result);
        }

        Json(json!(CreatePartResponse {
            status_code: http::StatusCode::OK,
            data: Some(format!("{} data entries successful", counter)),
            error: None
        }))
    }
}