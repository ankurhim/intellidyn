use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::naive::NaiveDateTime;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::bill_of_material::bill_of_material_model::BillOfMaterial;
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
    pub cut_weight: f64
}

impl CreateBillOfMaterialRequest {
    pub async fn create_bom(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
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
                gross_weight DOUBLE PRECEISION NOT NULL,
                cut_weight DOUBLE PRECISION NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                UNIQUE (challan_no, heat_no)
            )", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        let result = if !create_table.is_err() {
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
                modified_on
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            &[]
          )
          .await
          .map_err(|e| {
            dgb!(e);
            Err(AppError::DataInsertionFailed)
          })
        } else {
            Err(AppError::TableDoesNotExist)
        };

        Ok(Json(json!(result)))
    }
}