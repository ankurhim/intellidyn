use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::verify;
use axum::{
    Extension,
    Json,
    extract::Query
};
use time::{ Date, macros::{format_description, date}};

use serde_json::{Value, json};

use crate::routes::incoming_steel::incoming_steel_model::{IncomingSteel, SteelInventory};
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInventoryRequest {
    pub filter: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInventoryByDateRangeRequest {
    pub start_date: String,
    pub end_date: String
}

#[derive(Debug, Serialize)]
pub struct FindInventoryResponse {
    pub data: Vec<IncomingSteel>
}

impl FindInventoryRequest {
    pub async fn get_inventory(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let mut steel_vector: Vec<SteelInventory> = Vec::new();

        let resp = service.client
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
                    intellidyn_incoming_steel_table
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
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            steel_vector.push(SteelInventory {
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

        Ok(Json(json!(steel_vector)))
    }

    pub async fn get_inventory_by_filter(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindInventoryRequest>,
    ) -> Result<Json<Value>, AppError> {
        let mut steel_vector: Vec<SteelInventory> = Vec::new();

        let resp = service.client
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
                    intellidyn_incoming_steel_table
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
            &[&query.filter]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            steel_vector.push(SteelInventory {
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

        Ok(Json(json!(steel_vector)))
    }
}

impl FindInventoryByDateRangeRequest {
    pub async fn get_inventory_by_date_range(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(date_range): Query<FindInventoryByDateRangeRequest>
    ) -> Result<Json<Value>, AppError> {
        let mut steel_vector: Vec<SteelInventory> = Vec::new();

        let date_format = format_description!("%y-%m-%d");

        let start_date = Date::parse(&date_range.start_date, &date_format).unwrap();
        let end_date = Date::parse(&date_range.end_date, &date_format).unwrap();

        let resp = service.client
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
                    intellidyn_incoming_steel_table
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
            &[&date_range.start_date, &date_range.end_date]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            steel_vector.push(SteelInventory {
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

        Ok(Json(json!(steel_vector)))
    }
}