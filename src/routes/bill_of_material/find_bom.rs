use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Utc };
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;
use crate::routes::bill_of_material::bill_of_material_model::BillOfMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindBillOfMaterialRequest {
    pub filter: Option<String>
}

impl FindBillOfMaterialRequest {
    pub async fn find_bom_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Result<Json<Value>, AppError> {
        let find_table_result = service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_bill_of_material_table';",
            &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::InternalServerError
        });

        Ok(Json(json!(find_table_result)))
    }

    pub async fn find_all_boms(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Result<Json<Value>, AppError> {

        let mut bom_vector: Vec<BillOfMaterial> = Vec::new();

        if !Self::find_bom_table(Extension(logged_user.clone()), Extension(service.clone())).await.is_err() {
            let resp = service.client
            .query("SELECT * FROM mwspl_bill_of_material_table;", &[])
            .await
            .map_err(|e|{
                dbg!(e);
                AppError::InternalServerError
            })?;

            for row in resp {
                bom_vector.push(BillOfMaterial {
                    bom_pk: Uuid::parse_str(row.get(1)).unwrap(),
                    purchase_order_no: row.get(2),
                    po_date: row.get(3),
                    part_no: row.get(4),
                    part_name: row.get(5),
                    part_code: row.get(6),
                    grade: row.get(7),
                    section: row.get(8),
                    section_type: row.get(9),
                    jominy_range: row.get(10),
                    gross_weight: row.get(11),
                    cut_weight: row.get(12),
                    po_status: row.get(13),
                    created_by: row.get(14),
                    created_on: row.get(15),
                    modified_by: row.get(16),
                    modified_on: row.get(17),
                    remarks: row.get(18)
                })
            }
        }

        match &bom_vector.len() {
            0 => Ok(Json(json!(0))),
            _ => Ok(Json(json!(bom_vector)))
        }
    }
}