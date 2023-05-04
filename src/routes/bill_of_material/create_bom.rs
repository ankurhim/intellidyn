use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillOfMaterialRequest {
    pub part_no: String,
    pub part_name: String,
    pub part_code: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub remarks: Option<String>
}

impl CreateBillOfMaterialRequest {
    pub async fn create_bom_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_bill_of_material_table (
                ID SERIAL NOT NULL,
                bom_pk TEXT NOT NULL,
                part_no TEXT NOT NULL,
                part_name TEXT NOT NULL,
                part_code TEXT NOT NULL,
                grade TEXT NOT NULL,
                section BIGINT NOT NULL,
                section_type TEXT NOT NULL,
                jominy_range TEXT,
                gross_weight FLOAT8 NOT NULL,
                cut_weight FLOAT8 NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE (part_no, part_code, grade, section, section_type)
            );", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        Ok(Json(json!(create_table)))
    }

    pub async fn drop_bom_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let drop_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_bill_of_material_table;", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableDeletionFailed
        });

        Ok(Json(json!(drop_table)))
    }

    pub async fn create_bom(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Result<Json<Value>, AppError> {
        let result = if !Self::create_bom_table(Extension(logged_user.clone()), Extension(service.clone())).await.is_err() {
            service.client
            .execute(
                "INSERT INTO mwspl_bill_of_material_table (
                    bom_pk,
                    part_no,
                    part_name,
                    part_code,
                    grade,
                    section,
                    section_type,
                    jominy_range,
                    gross_weight,
                    cut_weight,
                    created_by,
                    created_on,
                    modified_by,
                    modified_on,
                    remarks
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15);",
            &[
                &Uuid::new_v4().to_string(),
                &payload.part_no,
                &payload.part_name,
                &payload.part_code,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
                &payload.gross_weight,
                &payload.cut_weight,
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