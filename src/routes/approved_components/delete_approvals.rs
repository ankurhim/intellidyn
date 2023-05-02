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
pub struct DeleteApprovedComponentRequest {
    pub heat_no: String,
    pub part_no: String
}

#[derive(Debug, Serialize)]
pub struct DeleteApprovedComponentResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl DeleteApprovedComponentRequest {
    pub async fn delete_part_by_filter(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<DeleteApprovedComponentRequest>,
    ) -> Result<Json<Value>, AppError> {
        let resp = service.client
        .execute(
            "DELETE FROM intellidyn_approved_component_table WHERE heat_no = $1 AND approved_part = $2", &[&query.heat_no, &query.part_no]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        Ok(Json(json!(resp)))
    }
}