use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{DateTime, Local};
use std::sync::Arc;
use axum::{
    Extension,
    Json
};

use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;

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
    pub received_qty: i64,
    pub remarks: Option<String>
}

#[derive(Debug, Serialize)]
pub struct CreateIncomingSteelResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateIncomingSteelRequest {
    pub async fn create_incoming_steel_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_incoming_steel_table (
                id SERIAL NOT NULL,
                incoming_pk TEXT NOT NULL,
                challan_no TEXT NOT NULL,
                challan_date DATE NOT NULL,
                grade TEXT NOT NULL,
                section INT NOT NULL,
                section_type TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                heat_code TEXT,
                jominy_value TEXT,
                received_qty BIGINT NOT NULL,
                issued_qty BIGINT NOT NULL,
                actual_qty BIGINT NOT NULL,
                heat_status TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (challan_no, heat_no)
            );", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        Ok(Json(json!(create_table)))
    }

    pub async fn drop_incoming_steel_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let drop_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_incoming_steel_table;", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableDeletionFailed
        });

        Ok(Json(json!(drop_table)))
    }

    pub async fn create_new_incoming_steel(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Result<Json<Value>, AppError> {

        let result = if !Self::create_incoming_steel_table(Extension(logged_user.clone()), Extension(service.clone())).await.is_err() {
            service.client
            .execute(
                "INSERT INTO mwspl_incoming_steel_table (
                    incoming_pk,
                    challan_no,
                    challan_date,
                    grade,
                    section,
                    section_type,
                    heat_no,
                    heat_code,
                    jominy_value,
                    received_qty,
                    issued_qty,
                    actual_qty,
                    heat_status,
                    created_by,
                    created_on,
                    modified_by,
                    modified_on,
                    remarks
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10,
                    $11,
                    $12,
                    $13,
                    $14,
                    $15,
                    $16,
                    $17,
                    $18
                )", &[
                    &Uuid::new_v4().to_string(),
                    &payload.challan_no,
                    &payload.challan_date,
                    &payload.grade,
                    &payload.section,
                    &payload.section_type,
                    &payload.heat_no,
                    &payload.heat_code,
                    &payload.jominy_value,
                    &payload.received_qty,
                    &0,
                    &payload.received_qty,
                    &Some(logged_user.username.clone()),
                    &Local::now(),
                    &None::<String>,
                    &None::<DateTime<Local>>,
                    &Some(payload.remarks)
                ]
            )
            .await
            .map_err(|e| {
              dbg!(e);
              AppError::DataInsertionFailed
            })?
          } else {
              0
          };
  
          Ok(Json(json!(result)))
    }
}