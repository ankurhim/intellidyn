use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{Local, DateTime};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::Path,
    http
};

use serde_json::{Value, json};

use crate::routes::rm_inventory::rm_inventory_model::RMInventory;
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRMInventoryRequest {
    pub heat_no: String,
    pub part_list: Vec<String>,
    pub remarks: Option<String>
}

impl CreateRMInventoryRequest {
    pub async fn create_rm_inventory() -> Json<Value> {
        service.client
        .execute(
            "SELECT DISTINCT
            a.heat_no,
            s.grade,
            s.size,
            s.section,
            a.avail_qty
            FROM mwspl_approved_component_table a
            INNER JOIN mwspl_steel_table s
            ON s.steel_code = (SELECT steel_code FROM incoming_steel WHERE incoming_steel_pk = a.rm_id);", &[]
        )
    }
}