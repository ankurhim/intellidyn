use serde::{Serialize, Deserialize };
use uuid::Uuid;
use time::{ Date, macros::{format_description, date}};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::incoming_steel::IncomingSteel;
use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSteelRequisitionRequest {
    pub part_no: String,
    pub cutting_quantity: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateSteelRequisitionResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateSteelRequisitionRequest {
    pub async fn create_new_requisition(

    ) -> Result<Json<Value>, AppError> {
        
    }
}