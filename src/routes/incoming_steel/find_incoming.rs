use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};

use serde_json::{Value, json};

use crate::service::DbService;
use crate::routes::incoming_steel::incoming_steel_model::IncomingSteel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindIncomingSteelRequest {
    pub filter: Option<String>
}

impl FindIncomingSteelRequest {
    pub async fn find_incoming_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_incoming_steel_table';",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn find_all_incoming_steels(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
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

        let mut steel_vector: Vec<IncomingSteel> = Vec::new();
        
        let resp = service.client
        .query("SELECT * FROM mwspl_incoming_steel_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_steel_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no:row.get(2),
                challan_date:row.get(3),
                grade:row.get(4),
                section:row.get(5),
                section_type:row.get(6),
                heat_no:row.get(7),
                heat_code:row.get(8),
                jominy_value:row.get(9),
                opening_qty:row.get(10),
                received_qty:row.get(11),
                issued_qty:row.get(12),
                actual_qty:row.get(13),
                heat_status:row.get(14),
                created_by: row.get(15),
                created_on: row.get(16),
                created_login_key: row.get(17),
                modified_by: row.get(18),
                modified_on: row.get(19),
                modified_login_key: row.get(20),
                remarks: row.get(21)
            })
        };
        match &steel_vector.len() {
            0 => Json(json!(None::<Vec<IncomingSteel>>)),
            _ => Json(json!(steel_vector))
        }
    }

    pub async fn find_all_incoming_steels_by_filter(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(payload): Query<FindIncomingSteelRequest>
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

        let mut steel_vector: Vec<IncomingSteel> = Vec::new();
        
        let resp = service.client
        .query("SELECT * FROM mwspl_incoming_steel_table WHERE challan_no = $1 OR grade = $1 OR heat_no = $1 OR heat_code = $1;", &[&payload.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_steel_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no:row.get(2),
                challan_date:row.get(3),
                grade:row.get(4),
                section:row.get(5),
                section_type:row.get(6),
                heat_no:row.get(7),
                heat_code:row.get(8),
                jominy_value:row.get(9),
                opening_qty:row.get(10),
                received_qty:row.get(11),
                issued_qty:row.get(12),
                actual_qty:row.get(13),
                heat_status:row.get(14),
                created_by: row.get(15),
                created_on: row.get(16),
                created_login_key: row.get(17),
                modified_by: row.get(18),
                modified_on: row.get(19),
                modified_login_key: row.get(20),
                remarks: row.get(21)
            })
        };
        match &steel_vector.len() {
            0 => Json(json!(None::<Vec<IncomingSteel>>)),
            _ => Json(json!(steel_vector))
        }
    }
}