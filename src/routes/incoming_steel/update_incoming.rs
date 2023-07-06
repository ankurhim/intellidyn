use serde::{Serialize, Deserialize };
use std::sync::Arc;
use axum::{
    Extension,
    Json
};

use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub drawing_no: String,
    pub issued_qty: i64,
}

impl UpdateInventoryRequest {
    pub async fn update_inventory(
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<UpdateInventoryRequest>,
    ) -> Json<Value> {
        match service.client
        .execute(
            "UPDATE mwspl_incoming_steel_table
            SET issued_qty = ($2 * (SELECT cut_weight FROM mwspl_bill_of_material_table WHERE drawing_no = $1)) :: BIGINT
            WHERE heat_no = (SELECT heat_no FROM mwspl_approved_component_table WHERE approved_part = $1) AND
            heat_status IS NULL;",
            &[&payload.drawing_no, &payload.issued_qty]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!{e.to_string()})) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}