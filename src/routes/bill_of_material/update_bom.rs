use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{ DateTime, Utc };
use axum::{
    Extension,
    Json,
    extract::Query
};
use chrono::Local;
use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;
use crate::routes::bill_of_material::bill_of_material_model::BillOfMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBillOfMaterialRequest {
    pub part_no: String,
    pub part_code: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayloadRequest {
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

impl UpdateBillOfMaterialRequest {
    pub async fn update_bom(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<UpdateBillOfMaterialRequest>,
        Json(payload): Json<UpdatePayloadRequest>
    ) -> Result<Json<Value>, AppError> {

        let update_query = service.client
        .execute(
            "UPDATE mwspl_bill_of_material_table
            SET
                part_no = $6,
                part_name = $7,
                part_code = $8,
                grade = $9,
                section = $10,
                section_type = $11,
                jominy_range = $12,
                gross_weight = $13,
                cut_weight = $14,
                remarks = $15,
                modified_by = $16,
                modified_on = $17
            WHERE
                part_no = $1
            AND
                part_code = $2
            AND
                grade = $3
            AND
                section = $4
            AND
                section_type = $5
            RETURNING *;",
            &[
                &query.part_no,
                &query.part_code,
                &query.grade,
                &query.section,
                &query.section_type,
                &payload.part_no,
                &payload.part_name,
                &payload.part_code,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
                &payload.gross_weight,
                &payload.cut_weight,
                &payload.remarks,
                &Some(logged_user.username.clone()),
                &Local::now(),
                ]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        Ok(Json(json!(update_query)))
    }
}