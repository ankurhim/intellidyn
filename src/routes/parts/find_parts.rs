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
use crate::routes::parts::part_model::Part;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindPartRequest {
    pub filter: Option<String>
}

impl FindPartRequest {
    pub async fn find_part_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {
        match service.client
        .execute(
            "SELECT * FROM information_schema.tables WHERE table_schema LIKE 'public' AND table_name = 'mwspl_part_table';",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn find_all_parts(
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
        
        let resp = service.client
        .query("SELECT
        p.id,
        part_pk,
        part_code,
        part_no,
        part_name,
        dwg_rev_no,
        p.steel_code,
        steel_grade,
        section,
        section_type,
        gross_weight,
        cut_weight,
        cut_length,
        part_status,
        p.created_by,
        p.created_on,
        p.created_login_key,
        p.modified_by,
        p.modified_on,
        p.modified_login_key
        FROM mwspl_part_table p
        INNER JOIN mwspl_steel_table s
        ON p.steel_code = s.steel_code
        WHERE part_status IS NULL;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_all_parts_by_filter(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(value): Query<FindPartRequest>
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
        .query("SELECT
        p.id,
        part_pk,
        part_code,
        part_no,
        part_name,
        dwg_rev_no,
        p.steel_code,
        steel_grade,
        section,
        section_type,
        gross_weight,
        cut_weight,
        cut_length,
        part_status,
        p.created_by,
        p.created_on,
        p.created_login_key,
        p.modified_by,
        p.modified_on,
        p.modified_login_key
        FROM mwspl_part_table p
        INNER JOIN mwspl_steel_table s
        ON p.steel_code = s.steel_code
        WHERE part_code = $1 OR part_grade = $1 AND part_status IS NULL;", &[&value.filter])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Part> = Vec::new();
    
    for row in row_vector {
        vector.push(Part {
            part_pk: Uuid::parse_str(row.get(1)).unwrap(),
            part_code: row.get(2),
            part_no: row.get(3),
            part_name: row.get(4),
            dwg_rev_no: row.get(5),
            steel_code: row.get(6),
            steel_grade: row.get(7),
            section: row.get(8),
            section_type: row.get(9),
            gross_weight: row.get(10),
            cut_weight: row.get(11),
            cut_length:row.get(12),
            part_status: row.get(13),
            created_by: row.get(14),
            created_on: row.get(15),
            created_login_key: row.get(16),
            modified_by: row.get(17),
            modified_on: row.get(18),
            modified_login_key: row.get(19)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Part>>)),
        _ => Json(json!(vector))
    }
}