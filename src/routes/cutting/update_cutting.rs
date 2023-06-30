use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use tokio_postgres::Row;
use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::cutting::cutting_model::Cutting;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCuttingProductionRequest {
    pub actual_qty: i64,
    pub ok_qty: i64,
    pub end_pc_wt: f64,
    pub rm_id: String,
    pub cutting_pk: String
}

impl UpdateCuttingProductionRequest {
    pub async fn update_production_details(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
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

        service.client
        .execute(
            "UPDATE mwpsl_cutting_table
            SET actual_qty = $1,
            ok_qty = $2,
            ok_wt = (SELECT cut_wt FROM mwspl_cutting_table WHERE cutting_pk = $5) * $2::FLOAT8,
            end_pc_wt = $3,
            rej_qty = $1 - $2,
            rej_wt = (SELECT cut_wt FROM mwspl_cutting_table WHERE cutting_pk = $5) * ($1 - $2)::FLOAT8,
            total_wt = (SELECT cut_wt FROM mwspl_cutting_table WHERE cutting_pk = $5) * $1::FLOAT8,
            WHERE rm_id = $4 AND cutting_pk = $5",
            &[
                &payload.actual_qty,
                &payload.ok_qty,
                &payload.end_pc_wt,
                &payload.rm_id,
                &payload.cutting_pk
            ]
        )
        .await
        .map(|v| {
            service.client
            .execute(
                "UPDATE mwspl_approved_component_table SET avail_qty = (avail_qty - (SELECT cut_wt FROM mwspl_cutting_table WHERE cutting_pk = $2) * $1::FLOAT8)
                WHERE rm_id = (SELECT DISTINCT rm_id FROM mwspl_cutting_table WHERE cutting_pk = $2);",
                &[
                    &payload.actual_qty,
                    &payload.cutting_pk
                ]
            )
            .await
            .map(|val| Json(json!(val)))
            .map_err(|err| Json(json!(e.to_string())))
        })
        .map_err(|e| Json(json!(e.to_string())))
    }
}