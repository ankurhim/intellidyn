use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;

use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::incoming_steel::incoming_steel_model::{IncomingSteel};
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindIncomingSteelRequest {
    pub filter: Option<String>
}

#[derive(Debug, Serialize)]
pub struct FindIncomingSteelResponse {
    pub data: Vec<IncomingSteel>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInventoryByDateRangeRequest {
    pub start_date: std::time::SystemTime,
    pub end_date: std::time::SystemTime
}

impl FindIncomingSteelRequest {
    pub async fn find_incoming_steels(
        Extension(_logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let mut steel_vector: Vec<IncomingSteel> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_incoming_steel_table", &[]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            steel_vector.push(IncomingSteel {
                incoming_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                grade: row.get(4),
                section: row.get(5),
                section_type: row.get(6),
                heat_no: row.get(7),
                heat_code: row.get(8),
                jominy_value: row.get(9),
                received_qty: row.get(10),
                issued_qty: row.get(11),
                actual_qty: row.get(12),
                heat_status: row.get(13),
                created_by: row.get(14),
                created_on: row.get(15),
                modified_by: row.get(16),
                modified_on: row.get(17)
            })
        }

        Ok(Json(json!(steel_vector)))
    }

    pub async fn find_incoming_steels_by_filter(
        Extension(_logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindIncomingSteelRequest>,
    ) -> Result<Json<Value>, AppError> {
        let mut steel_vector: Vec<IncomingSteel> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_incoming_steel_table WHERE challan_no = $1 OR grade = $1 OR heat_no = $1 OR heat_code = $1", &[&query.filter]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            steel_vector.push(IncomingSteel {
                incoming_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                grade: row.get(4),
                section: row.get(5),
                section_type: row.get(6),
                heat_no: row.get(7),
                heat_code: row.get(8),
                jominy_value: row.get(9),
                received_qty: row.get(10),
                issued_qty: row.get(11),
                actual_qty: row.get(12),
                heat_status: row.get(13),
                created_by: row.get(14),
                created_on: row.get(15),
                modified_by: row.get(16),
                modified_on: row.get(17)
            })
        }

        Ok(Json(json!(steel_vector)))
    }
}