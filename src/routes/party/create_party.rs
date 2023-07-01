use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartyRequest {
    pub party_id: String,
    pub party_type: String,
    pub party_name: String,
    pub party_address: String,
    pub gstn: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartyResponse {
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreatePartyRequest {
    pub async fn create_party_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_party_table(
                id SERIAL NOT NULL,
                party_pk TEXT NOT NULL,
                party_id TEXT NOT NULL PRIMARY KEY,
                party_type TEXT NOT NULL,
                party_name TEXT NOT NULL,
                party_address TEXT NOT NULL,
                gstn TEXT NOT NULL,
                party_status TEXT,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                UNIQUE (party_id, gstn)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(CreatePartyResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartyResponse {
            data: None,
            error: Some(err.to_string())
        }))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_party_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_party_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(CreatePartyResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartyResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_party(
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
        
        match service.client
        .execute(
            "INSERT INTO mwspl_party_table(
                party_pk,
                party_id,
                party_type,
                party_name,
                party_address,
                gstn,
                party_status,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 , $13)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.party_id,
                &payload.party_type,
                &payload.party_name,
                &payload.party_address,
                &payload.gstn,
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
        .map(|val| Json(json!(CreatePartyResponse {
            data: Some(val.to_string()),
            error: None
        })))
        .map_err(|err| Json(json!(CreatePartyResponse {
            data: None,
            error: Some(err.to_string())
        })))  {
            Ok(v) => v,
            Err(e) => e
        }
    }
}