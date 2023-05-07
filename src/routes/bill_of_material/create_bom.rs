use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
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
    pub purchase_order_no: String,
    pub po_date: String,
    pub part_no: String,
    pub part_name: String,
    pub part_code: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub po_status: String,
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
                purchase_order_no TEXT NOT NULL,
                po_date DATE NOT NULL,
                part_no TEXT NOT NULL,
                part_name TEXT NOT NULL,
                part_code TEXT NOT NULL,
                grade TEXT NOT NULL,
                section BIGINT NOT NULL,
                section_type TEXT NOT NULL,
                jominy_range TEXT,
                gross_weight FLOAT8 NOT NULL,
                cut_weight FLOAT8 NOT NULL,
                po_status TEXT NOT NULL,
                created_by TEXT NOT NULL,
                created_on TIMESTAMPTZ NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMPTZ,
                remarks TEXT,
                UNIQUE  (purchase_order_no, part_no, part_code, grade, section, section_type)
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

        let parsed_po_date = NaiveDate::parse_from_str(&payload.po_date, "%d-%m-%Y").expect("Date parsing error");

        let result = service.client
            .execute(
                "INSERT INTO mwspl_bill_of_material_table (
                    bom_pk,
                    purchase_order_no,
                    po_date,
                    part_no,
                    part_name,
                    part_code,
                    grade,
                    section,
                    section_type,
                    jominy_range,
                    gross_weight,
                    cut_weight,
                    po_status,
                    created_by,
                    created_on,
                    modified_by,
                    modified_on,
                    remarks
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18);",
            &[
                &Uuid::new_v4().to_string(),
                &payload.purchase_order_no,
                &parsed_po_date,
                &payload.part_no,
                &payload.part_name,
                &payload.part_code,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
                &payload.gross_weight,
                &payload.cut_weight,
                &payload.po_status,
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
          });

        Ok(Json(json!(result)))
    }
}