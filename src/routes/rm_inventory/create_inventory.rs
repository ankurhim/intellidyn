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
pub struct CreateRMInventoryRequest;

impl CreateRMInventoryRequest {
    pub async fn create_rm_inventory(    
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut inventory_vector: Vec<RMInventory> = Vec::new();

        let resp = service.client
        .query(
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
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            inventory_vector.push(RMInventory {
                heat_no: row.get(1),
                grade: row.get(2),
                size: row.get(3),
                section: row.get(4),
                avail_qty: row.get(5)
            })
        }

        Json(json!(inventory_vector))
    }
}