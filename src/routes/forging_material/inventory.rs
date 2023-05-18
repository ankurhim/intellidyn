use serde::{Serialize, Deserialize };
use uuid::Uuid;
use chrono::{DateTime, naive::NaiveDate, Local};
use axum::{Extension, Json, extract::{Path, Query}};
use serde_json::{Value, json};
use crate::routes::DbService;
use std::sync::Arc;
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub drawing_no: String,
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub total_available_qty: i64,
    pub total_available_weight: f64
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
                drawing_no,
                heat_no,
                grade,
                section,
                section_type,
                SUM(available_qty) :: BIGINT AS total_available_qty,
                SUM(available_qty * cut_weight) :: FLOAT8 AS total_available_weight
            FROM
                mwspl_forging_material_table
            WHERE
                batch_status = 'NOT ISSUED'
            GROUP BY
                drawing_no,
                heat_no,
                grade,
                section,
                section_type
            ORDER BY
                forging_date;",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(result.unwrap())
    }

    pub async fn fetch_inventory_by_filter(
        Path((user, login_key)): Path<(String,String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindInventoryRequest>,
    )-> Json<Value> {
        let result = service.client
        .query(
            "SELECT
                drawing_no,
                heat_no,
                grade,
                section,
                section_type,
                SUM(available_qty) :: BIGINT AS total_available_qty,
                SUM(available_qty * cut_weight) :: FLOAT8 AS total_available_weight
            FROM
                mwspl_forging_material_table
            WHERE
                drawing_no = $1 OR
                grade = $1 OR 
                heat_no = $1 AND
                batch_status = 'NOT ISSUED'
            GROUP BY
                drawing_no,
                heat_no,
                grade,
                section,
                section_type
            ORDER BY
                forging_date;",
            &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(result.unwrap())
    }

    pub async fn fetch_inventory_filtered_by_date(
        Path((user, login_key)): Path<(String,String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(date_range): Query<FindInventoryByDateRangeRequest>
    )-> Json<Value> {

        let start_date = NaiveDate::parse_from_str(&date_range.start_date, "%d-%m-%Y").expect("PO Date parsing error");
        let end_date = NaiveDate::parse_from_str(&date_range.end_date, "%d-%m-%Y").expect("PO Date parsing error");

        let result = service.client
        .query(
            "SELECT
                drawing_no,
                heat_no,
                grade,
                section,
                section_type,
                SUM(available_qty) :: BIGINT AS total_available_qty,
                SUM(available_qty * cut_weight) :: FLOAT8 AS total_available_weight
            FROM
                mwspl_forging_material_table
            WHERE
                forging_date IS BETWEEN $1 AND $2
                batch_status = 'NOT ISSUED'
            GROUP BY
                drawing_no,
                heat_no,
                grade,
                section,
                section_type
            ORDER BY
                forging_date;",
            &[&start_date, &end_date]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(result.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<Inventory> = Vec::new();
    
    for row in row_vector {
        vector.push(Inventory {
            drawing_no: row.get(0),
            heat_no: row.get(1),
            grade: row.get(2),
            section: row.get(3),
            section_type: row.get(4),
            total_available_qty: row.get(5),
            total_available_weight: row.get(6)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<Inventory>>)),
        _ => Json(json!(vector))
    }
}