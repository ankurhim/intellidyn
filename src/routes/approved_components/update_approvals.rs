use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use axum::{
    Extension,
    Json,
    extract::Query
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApprovedComponentRequest {
    pub heat_no: String,
    pub approved_part: String
}

#[derive(Debug, Serialize)]
pub struct UpdateApprovedComponentResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl UpdateApprovedComponentRequest {
    pub async fn update_approved_part_by_heat_no(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(query): Json<UpdateApprovedComponentRequest>,
    ) -> Result<Json<Value>, AppError> {
        
        let resp = service.client
        .execute(
            "UPDATE intellidyn_approved_component_table SET approved_part = $2, modified_by = $3, modified_on = $4 WHERE heat_no = $1;", &[
                &query.heat_no,
                &query.approved_part,
                &logged_user.username,
                &Some(std::time::SystemTime::now())
                ]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        Ok(Json(json!(resp)))
    }
}