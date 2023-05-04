use serde::{Serialize, Deserialize };

use std::sync::Arc;

use axum::{
    Extension,
    Json
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApprovedComponentTableRequest {
    pub heat_no: String,
    pub section: i64,
    pub section_type: String
}

#[derive(Debug, Serialize)]
pub struct UpdateApprovedComponentResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl UpdateApprovedComponentRequest {
    pub async fn update_approved_component_by_heat_no(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(query): Json<UpdateApprovedComponentRequest>,
    ) -> Result<Json<Value>, AppError> {
        
        let resp = service.client
        .execute(
            "UPDATE intellidyn_approved_component_table SET section = $2, modified_by = $3, modified_on = $4 WHERE heat_no = $1;", &[
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

impl UpdateApprovedComponentTableRequest {
    pub async fn update_section_by_heat_no(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(query): Json<UpdateApprovedComponentTableRequest>,
    ) -> Result<Json<Value>, AppError> {
        
        let resp = service.client
        .execute(
            "UPDATE intellidyn_approved_component_table SET section = $2, section_type = $3, modified_by = $4, modified_on = $5 WHERE heat_no = $1;", &[
                &query.heat_no,
                &query.section,
                &query.section_type,
                &logged_user.username,
                &Some(std::time::SystemTime::now())
                ]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        let _ = service.client
        .execute("ALTER TABLE intellidyn_approved_component_table
            ALTER COLUMN section SET NOT NULL,
            ALTER COLUMN section_type SET NOT NULL;", &[]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        });

        Ok(Json(json!(resp)))
    }
}