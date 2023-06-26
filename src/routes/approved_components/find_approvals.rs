use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use tokio_postgres::Row;
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};

use serde_json::{Value, json};

use crate::routes::approved_components::approved_components_model::ApprovedComponent;
use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindApprovedHeatsRequest {
    pub filter: Option<String>
}

#[derive(Debug, Serialize)]
pub struct FindApprovedHeatsResponse {
    pub data: Vec<ApprovedComponent>
}

impl FindApprovedHeatsRequest {
    pub async fn find_approved_heats(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_approved_component_table", &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }

    pub async fn find_approved_heats_by_filter(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindApprovedHeatsRequest>,
    ) -> Json<Value> {

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_approved_component_table WHERE heat_no = $1 OR approved_part = $1", &[&query.filter]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        get_list(resp.unwrap())
    }
}

fn get_list(row_vector: Vec<Row>) -> Json<Value> {
    
    let mut vector: Vec<ApprovedComponent> = Vec::new();
    
    for row in row_vector {
        vector.push(ApprovedComponent {
            approval_pk: Uuid::parse_str(row.get(1)).unwrap(),
            heat_no: row.get(2),
            approved_part: row.get(6),
            created_by: row.get(7),
            created_on: row.get(8),
            created_login_key: row.get(9),
            modified_by: row.get(10),
            modified_on: row.get(11),
            modified_login_key: row.get(12)
        })
    };
    match &vector.len() {
        0 => Json(json!(None::<Vec<ApprovedComponent>>)),
        _ => Json(json!(vector))
    }
}