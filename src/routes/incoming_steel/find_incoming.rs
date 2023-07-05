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
        .query("SELECT
        i.id,
        incoming_steel_pk,
        challan_no,
        challan_date,
        i.steel_code,
        steel_grade,
        section,
        section_type,
        heat_no,
        heat_code,
        jominy_value,
        received_qty,
        avail_qty,
        heat_status,
        i.created_by,
        i.created_on,
        i.created_login_key,
        i.modified_by,
        i.modified_on,
        i.modified_login_key
        FROM mwspl_incoming_steel_table i
        INNER JOIN mwspl_steel_table s
        ON i.steel_code = s.steel_code
        ORDER BY challan_no ASC, challan_date ASC;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_steel_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                steel_code: row.get(4),
                steel_grade: row.get(5),
                section: row.get(6),
                section_type: row.get(7),
                heat_no: row.get(8),
                heat_code: row.get(9),
                jominy_value: row.get(10),
                received_qty: row.get(11),
                avail_qty: row.get(12),
                heat_status: row.get(13),
                created_by: row.get(14),
                created_on: row.get(15),
                created_login_key: row.get(16),
                modified_by: row.get(17),
                modified_on: row.get(18),
                modified_login_key: row.get(19)
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
        .query("SELECT
        i.id,
        incoming_steel_pk,
        challan_no,
        challan_date,
        i.steel_code,
        steel_grade,
        section,
        section_type,
        heat_no,
        heat_code,
        jominy_value,
        received_qty,
        avail_qty,
        heat_status,
        i.created_by,
        i.created_on,
        i.created_login_key,
        i.modified_by,
        i.modified_on,
        i.modified_login_key
        FROM mwspl_incoming_steel_table i
        INNER JOIN mwspl_steel_table s
        ON i.steel_code = s.steel_code
        WHERE i.challan_no ILIKE $1 OR s.steel_grade ILIKE $1 OR i.heat_no ILIKE $1 OR i.heat_code ILIKE $1 AND heat_status IS NULL;", &[&format!("%{}%", &payload.filter.unwrap())])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_steel_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                steel_code: row.get(4),
                steel_grade: row.get(5),
                section: row.get(6),
                section_type: row.get(7),
                heat_no: row.get(8),
                heat_code: row.get(9),
                jominy_value: row.get(10),
                received_qty: row.get(11),
                avail_qty: row.get(12),
                heat_status: row.get(13),
                created_by: row.get(14),
                created_on: row.get(15),
                created_login_key: row.get(16),
                modified_by: row.get(17),
                modified_on: row.get(18),
                modified_login_key: row.get(19)
            })
        };
        match &steel_vector.len() {
            0 => Json(json!(None::<Vec<IncomingSteel>>)),
            _ => Json(json!(steel_vector))
        }
    }

    pub async fn get_heat_nos_list( 
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

        let mut heat_nos: Vec<String> = Vec::new();
        
        let resp = service.client
        .query("SELECT DISTINCT heat_no FROM mwspl_incoming_steel_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            heat_nos.push(row.get(1))
        };
        match &heat_nos.len() {
            0 => Json(json!(None::<Vec<String>>)),
            _ => Json(json!(heat_nos))
        }
    }
}