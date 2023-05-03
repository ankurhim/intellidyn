use serde::{Serialize, Deserialize };
use uuid::Uuid;
use time::{ Date, macros::{format_description, date}};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::cutting_production::cutting_request_model::SteelRequisition;
use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelRequisitionRequest {
    pub part_no: String,
    pub cutting_quantity: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateSteelRequisitionResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateSteelRequisitionRequest {
    pub async fn create_new_requisition(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Result<Json<Value>, AppError> {
        let create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS intellidyn_steel_requisition_table (
                id SERIAL NOT NULL,
                request_pk TEXT NOT NULL,
                part_no TEXT NOT NULL,
                cutting_qty BIGINT NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMP NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMP,
                UNIQUE (request_pk, part_no)
            );",
            &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        let result = if !create_table.is_err() {

            let new_request = SteelRequisition {
                request_pk: Uuid::new_v4(),
                part_no: payload.part_no,
                cutting_quantity: payload.cutting_quantity,
                created_by: Some(logged_user.username.to_string()),
                created_on: std::time::SystemTime::now(),
                modified_by: None,
                modified_on: None
            };

            let insert_result = service.client
            .execute(
                "INSERT INTO intellidyn_steel_requisition_table (
                    request_pk,
                    part_no,
                    cutting_qty,
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
                    $7
                );",
                &[
                    &new_request.request_pk.clone(),
                    &new_request.part_no,
                    &new_request.cutting_quantity,
                    match &new_request.created_by {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    &new_request.created_on,
                    match &new_request.modified_by {
                        Some(v) => v,
                        _ => &None::<String>
                    },
                    &new_request.modified_on
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

    pub async fn drop_steel_request_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let result = service.client
        .execute("DROP TABLE IF EXISTS intellidyn_steel_requisition_table;", &[])
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::InternalServerError
        });

        Ok(Json(json!(result)))
    }
}