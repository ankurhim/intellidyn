use serde::{Serialize, Deserialize };
use uuid::Uuid;
use time::{ Date, macros::{format_description, date}};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::incoming_steel::IncomingSteel;
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
    pub received_qty: i64
}

#[derive(Debug, Serialize)]
pub struct CreateIncomingSteelResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateIncomingSteelRequest {
    pub async fn create_new_incoming_steel(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Result<Json<Value>, AppError> {
        
        let _create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS intellidyn_incoming_steel_table (
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
                created_on TIMESTAMP NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMP,
                UNIQUE (challan_no, heat_no)
            );", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        let result = if !_create_table.is_err() {
            let date_format = format_description!("[day]-[month]-[year]");

            let new_incoming_steel = IncomingSteel {
                incoming_pk: Uuid::new_v4(),
                challan_no: payload.challan_no.clone(),
                challan_date: Date::parse(&payload.challan_date, date_format).unwrap(),
                grade: payload.grade.clone(),
                section: payload.section.clone(),
                section_type: payload.section_type.clone(),
                heat_no: payload.heat_no.clone(),
                heat_code: payload.heat_code.clone(),
                jominy_value: payload.jominy_value.clone(),
                received_qty: payload.received_qty.clone(),
                issued_qty: 0,
                actual_qty: payload.received_qty.clone() - 0,
                heat_status: None,
                created_by: Some(logged_user.username.to_string()),
                created_on: std::time::SystemTime::now(),
                modified_by: None,
                modified_on: None,
            };
    
            let insert_result = service.client
            .execute(
                "INSERT INTO intellidyn_incoming_steel_table (
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
                    modified_on
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
                    $17
                )", &[
                    &new_incoming_steel.incoming_pk.to_string(),
                    &new_incoming_steel.challan_no,
                    &new_incoming_steel.challan_date,
                    &new_incoming_steel.grade,
                    &new_incoming_steel.section,
                    &new_incoming_steel.section_type,
                    &new_incoming_steel.heat_no,
                    match &new_incoming_steel.heat_code {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    match &new_incoming_steel.jominy_value {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    &new_incoming_steel.received_qty,
                    &new_incoming_steel.issued_qty,
                    &new_incoming_steel.actual_qty,
                    match &new_incoming_steel.heat_status {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    match &new_incoming_steel.created_by {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    &new_incoming_steel.created_on,
                    match &new_incoming_steel.modified_by {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    &new_incoming_steel.modified_on
                ]
            )
            .await
            .map_err(|e| {
                dbg!(e);
                AppError::DataInsertionFailed
            })?;

            if insert_result < 1 {
                Err(AppError::DataInsertionFailed)
            } else {
                Ok(Json(json!(insert_result)))
            }
        } else {
            Err(AppError::TableDoesNotExist)
        };

        result
    }

    pub async fn drop_steel_incoming_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let result = service.client
        .execute("DROP TABLE IF EXISTS intellidyn_incoming_steel_table;", &[])
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::InternalServerError
        });

        Ok(Json(json!(result)))
    }
}