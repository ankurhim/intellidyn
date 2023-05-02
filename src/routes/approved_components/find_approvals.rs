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
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Result<Json<Value>, AppError> {
        let mut part_vector: Vec<ApprovedComponent> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_approved_component_table", &[]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            part_vector.push(ApprovedComponent {
                approval_pk: Uuid::parse_str(row.get(1)).unwrap(),
                heat_no: row.get(2),
                approved_part: row.get(3),
                created_by: row.get(4),
                created_on: row.get(5),
                modified_by: row.get(6),
                modified_on: row.get(7)
            })
        }

        Ok(Json(json!(part_vector)))
    }

    pub async fn find_incoming_steels_by_filter(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindApprovedHeatsRequest>,
    ) -> Result<Json<Value>, AppError> {
        let mut part_vector: Vec<ApprovedComponent> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM intellidyn_approved_component_table WHERE heat_no = $1 OR approved_part = $1", &[&query.filter]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        for row in resp {
            part_vector.push(ApprovedComponent {
                approval_pk: Uuid::parse_str(row.get(1)).unwrap(),
                heat_no: row.get(2),
                approved_part: row.get(3),
                created_by: row.get(4),
                created_on: row.get(5),
                modified_by: row.get(6),
                modified_on: row.get(7)
            })
        }

        Ok(Json(json!(part_vector)))
    }
}