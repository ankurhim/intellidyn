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
pub struct DeleteBillOfMaterialRequest {
    pub part_no: String,
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
                part_no = $2,
                part_name = $3,
                part_code = $4,
                grade = $5,
                section = $6,
                section_type = $7,
                jominy_range = $8,
                gross_weight = $9,
                cut_weight = $10,
                remarks = $11,
                modified_by = $12,
                modified_on = $13
            WHERE part_no = $1;",
            &[
                &query.part_no,
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
                &Local::now()
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