use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::verify;
use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::incoming_steel::incoming_steel_model::IncomingSteel;
use crate::routes::users::user_model::User;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindIncomingSteelRequest {
    pub heat_no: Option<String>
}

#[derive(Debug, Serialize)]
pub struct FindIncomingSteelResponse {
    pub data: Vec<IncomingSteel>
}

impl FindIncomingSteelRequest {
    pub async fn find_incoming_steels(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let mut steel_vector: Vec<IncomingSteel> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_incoming_material", &[]
        )
        .await
        .map_err(|e| Json(json!(FindIncomingSteelResponse {
            data: vec![]
        })));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                grade: row.get(4),
                section: row.get(5),
                heat_no: row.get(6),
                heat_code: row.get(7),
                jominy_value: row.get(8),
                received_qty: row.get(9),
                created_by: row.get(10),
                created_on: row.get(11),
                modified_by: row.get(12),
                modified_on: row.get(13)
            })
        }

        Json(json!(steel_vector))
    }

    pub async fn find_incoming_steels_by_heat_no(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindIncomingSteelRequest>,
    ) -> Json<Value> {
        let mut steel_vector: Vec<IncomingSteel> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_incoming_material WHERE heat_no = $1", &[&query.heat_no]
        )
        .await
        .map_err(|e| Json(json!(FindIncomingSteelResponse {
            data: vec![],
        })));

        for row in resp.unwrap() {
            steel_vector.push(IncomingSteel {
                incoming_pk: Uuid::parse_str(row.get(1)).unwrap(),
                challan_no: row.get(2),
                challan_date: row.get(3),
                grade: row.get(4),
                section: row.get(5),
                heat_no: row.get(6),
                heat_code: row.get(7),
                jominy_value: row.get(8),
                received_qty: row.get(9),
                created_by: row.get(10),
                created_on: row.get(11),
                modified_by: row.get(12),
                modified_on: row.get(13)
            })
        }

        Json(json!(FindIncomingSteelResponse {
            data: steel_vector,
        }))
    }
}