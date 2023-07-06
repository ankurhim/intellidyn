use serde::{Serialize, Deserialize };
use axum::{Extension, Json, extract::{Path, Query}};
use serde_json::{Value, json};
use crate::routes::DbService;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub heat_code: Option<String>,
    pub total_received_qty: i64,
    pub total_issued_qty: i64,
    pub total_available_qty: i64,
    pub heat_status: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInventoryRequest {
    pub filter: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInventoryByDateRangeRequest {
    pub start_date: String,
    pub end_date: String
}

impl Inventory {
    pub async fn fetch_inventory(
        Path((user, login_key)): Path<(String,String)>,
        Extension(service): Extension<Arc<DbService>>
    )-> Json<Value> {
        let result = service.client
        .query(
            "SELECT
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                SUM(received_qty) :: BIGINT AS total_received_qty,
                SUM(issued_qty) :: BIGINT AS total_issued_qty,
                SUM(received_qty - issued_qty) :: BIGINT AS total_available_qty,
                heat_status
            FROM
                mwspl_incoming_steel_table
            WHERE
                heat_status IS NULL
            GROUP BY
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                heat_status;",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        let mut res_vec: Vec<Inventory> = Vec::new();

        for row in result.unwrap() {
            res_vec.push(Inventory {
                heat_no: row.get(0),
                grade: row.get(1),
                section: row.get(2),
                section_type: row.get(3),
                heat_code: row.get(4),
                total_received_qty: row.get(5),
                total_issued_qty: row.get(6),
                total_available_qty: row.get(7),
                heat_status: row.get(8)
            })
        }

        Json(json!(res_vec))
    }

    pub async fn fetch_inventory_by_filter(
        Path((user, login_key)): Path<(String,String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindInventoryRequest>,
    )-> Json<Value> {
        let result = service.client
        .query(
            "SELECT
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                SUM(received_qty) :: BIGINT AS total_received_qty,
                SUM(issued_qty) :: BIGINT AS total_issued_qty,
                SUM(received_qty - issued_qty) :: BIGINT AS total_available_qty,
                heat_status
            FROM
                mwspl_incoming_steel_table
            WHERE
                challan_no = $1 OR
                grade = $1 OR 
                heat_no = $1 OR 
                heat_code = $1 AND
                heat_status IS NULL
            GROUP BY
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                heat_status;",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        let mut res_vec: Vec<Inventory> = Vec::new();

        for row in result.unwrap() {
            res_vec.push(Inventory {
                heat_no: row.get(0),
                grade: row.get(1),
                section: row.get(2),
                section_type: row.get(3),
                heat_code: row.get(4),
                total_received_qty: row.get(5),
                total_issued_qty: row.get(6),
                total_available_qty: row.get(7),
                heat_status: row.get(8)
            })
        }

        Json(json!(res_vec))
    }

    pub async fn fetch_inventory_filtered_by_date(
        Path((user, login_key)): Path<(String,String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(date_range): Query<FindInventoryByDateRangeRequest>
    )-> Json<Value> {
        let result = service.client
        .query(
            "SELECT
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                SUM(received_qty) :: BIGINT AS total_received_qty,
                SUM(issued_qty) :: BIGINT AS total_issued_qty,
                SUM(received_qty - issued_qty) :: BIGINT AS total_available_qty,
                heat_status
            FROM
                mwspl_incoming_steel_table
            WHERE
                challan_date BETWEEN $1 AND $2
                AND heat_status IS NULL
            GROUP BY
                heat_no,
                grade,
                section,
                section_type,
                heat_code,
                heat_status;",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        let mut res_vec: Vec<Inventory> = Vec::new();

        for row in result.unwrap() {
            res_vec.push(Inventory {
                heat_no: row.get(0),
                grade: row.get(1),
                section: row.get(2),
                section_type: row.get(3),
                heat_code: row.get(4),
                total_received_qty: row.get(5),
                total_issued_qty: row.get(6),
                total_available_qty: row.get(7),
                heat_status: row.get(8)
            })
        }

        Json(json!(res_vec))
    }
}