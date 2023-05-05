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
    pub bom_pk: String
}

impl DeleteBillOfMaterialRequest {
    pub async fn delete_bom(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<DeleteBillOfMaterialRequest>
    ) -> Result<Json<Value>, AppError> {

        let delete_query = service.client
        .execute(
            "DELETE FROM 
                mwspl_bill_of_material_table
            WHERE bom_pk = $1;",
            &[&query.bom_pk]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        Ok(Json(json!(delete_query)))
    }
}