use serde::{Serialize, Deserialize };

use std::sync::Arc;

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
pub struct DeleteIncomingSteelRequest {
    pub challan_no: String,
    pub heat_no: String
}

#[derive(Debug, Serialize)]
pub struct DeleteIncomingSteelResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl DeleteIncomingSteelRequest {
    pub async fn delete_steel_by_filter(
        Extension(_logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<DeleteIncomingSteelRequest>,
    ) -> Result<Json<Value>, AppError> {
        let resp = service.client
        .execute(
            "DELETE FROM intellidyn_incoming_steel_table WHERE challan_no = $1 AND heat_no = $2", &[&query.challan_no, &query.heat_no]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        })?;

        Ok(Json(json!(resp)))
    }
}