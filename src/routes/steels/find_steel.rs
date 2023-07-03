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
use crate::routes::steels::steel_model::Steel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSteelRequest {
    pub filter: Option<String>
}

impl FindSteelRequest {
    pub async fn find_steel_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_bom_table';",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn find_all_steels(
        // Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        // let resp = service.client
        // .query(
        //     "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        // )
        // .await
        // .map_err(|e| Json(json!(e.to_string())));

        // for row in resp.unwrap() {
        //     if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
        //         break;
        //     } else {
        //         return Json(json!("You are logged out"));
        //     }
        // }
        
        let resp = service.client
        .query("SELECT * FROM mwspl_steel_table WHERE steel_status IS NULL;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_all_steels_by_filter(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(value): Query<FindSteelRequest>
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
        
        let resp = service.client
        .query("SELECT * FROM mwspl_steel_table WHERE steel_code = $1 OR steel_grade = $1 AND steel_status IS NULL;", &[&value.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Steel> = Vec::new();
    
    for row in row_vector {
        vector.push(Steel {
            steel_pk: Uuid::parse_str(row.get(1)).unwrap(),
            steel_code: row.get(2),
            steel_grade: row.get(3),
            section: row.get(4),
            section_type: row.get(5),
            jominy_range: row.get(6),
            steel_status: row.get(7),
            created_by: row.get(8),
            created_on: row.get(9),
            created_login_key: row.get(10),
            modified_by: row.get(11),
            modified_on: row.get(12),
            modified_login_key: row.get(13)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Steel>>)),
        _ => Json(json!(vector))
    }
}