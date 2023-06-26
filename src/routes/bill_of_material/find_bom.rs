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
use crate::routes::bill_of_material::bom_model::BillOfMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindBillOfMaterialRequest {
    pub filter: Option<String>
}

impl FindBillOfMaterialRequest {
    pub async fn find_po_table(
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

    pub async fn find_all_boms(
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
        .query("SELECT * FROM mwspl_bill_of_material_table;", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    // pub async fn find_all_boms_by_dwg_no(
    //     Path((user, login_key)): Path<(String, String)>,
    //     Extension(service): Extension<Arc<DbService>>,
    //     Query(payload): Query<FindBillOfMaterialRequest>
    // ) -> Json<Value> {

    //     let resp = service.client
    //     .query(
    //         "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
    //     )
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     for row in resp.unwrap() {
    //         if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
    //             break;
    //         } else {
    //             return Json(json!("You are logged out"));
    //         }
    //     }
        
    //     let resp = service.client
    //     .query("SELECT * FROM mwspl_bom_table WHERE drawing_no = $1;", &[&payload.filter])
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     get_list(resp.unwrap())
    // }

    pub async fn find_active_boms(
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
        .query("SELECT * FROM mwspl_bom_table WHERE po_status = 'ACTIVE';", &[])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    // pub async fn find_active_boms_by_dwg_no(
    //     Path((user, login_key)): Path<(String, String)>,
    //     Extension(service): Extension<Arc<DbService>>,
    //     Query(payload): Query<FindBillOfMaterialRequest>
    // ) -> Json<Value> {

    //     let resp = service.client
    //     .query(
    //         "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
    //     )
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     for row in resp.unwrap() {
    //         if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
    //             break;
    //         } else {
    //             return Json(json!("You are logged out"));
    //         }
    //     }
        
    //     let resp = service.client
    //     .query("SELECT * FROM mwspl_bom_table WHERE drawing_no = $1 AND po_status = 'ACTIVE';", &[&payload.filter])
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     get_list(resp.unwrap())
    // }

    // pub async fn find_all_dwg_no(
    //     Path((user, login_key)): Path<(String, String)>,
    //     Extension(service): Extension<Arc<DbService>>
    // ) -> Json<Value> {

    //     let resp = service.client
    //     .query(
    //         "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
    //     )
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     for row in resp.unwrap() {
    //         if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
    //             break;
    //         } else {
    //             return Json(json!("You are logged out"));
    //         }
    //     }
        
    //     let resp = service.client
    //     .query("SELECT DISTINCT drawing_no, gross_weight, cut_weight FROM mwspl_bom_table WHERE po_status = 'ACTIVE';", &[])
    //     .await
    //     .map_err(|e| Json(json!(e.to_string())));

    //     let mut part_list: Vec<(String, f64, f64)> = Vec::new();

    //     for row in resp.unwrap() {
    //         part_list.push((row.get(0), row.get(1), row.get(2)))
    //     }

    //     Json(json!(part_list))
    // }

    pub async fn find_by_dwg_no(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(dwg_no): Query<String>
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
        .query("SELECT DISTINCT drawing_no, gross_weight, cut_weight FROM mwspl_bom_table WHERE drawing_no = $1 AND po_status = 'ACTIVE';", &[&dwg_no])
        .await
        .map_err(|e| Json(json!(e.to_string())));

        let mut part_list: Vec<(String, f64, f64)> = Vec::new();

        for row in resp.unwrap() {
            part_list.push((row.get(0), row.get(1), row.get(2)))
        }

        Json(json!(part_list))
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<BillOfMaterial> = Vec::new();
    
    for row in row_vector {
        vector.push(BillOfMaterial {
            purchase_order_pk: Uuid::parse_str(row.get(1)).unwrap(),
            purchase_order_no: row.get(2),
            po_date: row.get(3),
            party_id: row.get(4),
            po_quantity: row.get(5),
            po_received_date: row.get(6),
            po_effective_date: row.get(7),
            po_status: row.get(8),
            po_deactive_date: row.get(9),
            rate: row.get(10),
            part_code: row.get(11),
            steel_code: row.get(12),
            created_by: row.get(13),
            created_on: row.get(14),
            created_login_key: row.get(15),
            modified_by: row.get(16),
            modified_on: row.get(17),
            modified_login_key: row.get(18)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<BillOfMaterial>>)),
        _ => Json(json!(vector))
    }
}