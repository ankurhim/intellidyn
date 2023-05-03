use serde::{Serialize, Deserialize };
use uuid::Uuid;
use time::{ Date, macros::{format_description, date}};
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::approved_components::ApprovedComponent;
use crate::routes::User;
use crate::service::DbService;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApprovedComponentRequest {
    pub heat_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub part_list: Vec<String>
}

#[derive(Debug, Serialize)]
pub struct CreateApprovedComponentResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>
}

impl CreateApprovedComponentRequest {
    pub async fn create_new_approved_component(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
    ) -> Result<Json<Value>, AppError> {

        let _create_table = service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS intellidyn_approved_component_table (
                id SERIAL NOT NULL,
                approval_pk TEXT NOT NULL,
                heat_no TEXT NOT NULL,
                grade TEXT NOT NULL,
                section INT NOT NULL,
                section_type TEXT NOT NULL,
                approved_part TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMP NOT NULL,
                modified_by TEXT,
                modified_on TIMESTAMP,
                UNIQUE (heat_no, grade, section, section_type, approved_part)
            );", &[]
        )
        .await
        .map_err(|e| {
            dbg!(e);
            AppError::TableCreationFailed
        });

        let result = if !_create_table.is_err() {
            let row: i64 = service.client
            .query(
                "SELECT COUNT(*) FROM intellidyn_incoming_steel_table WHERE heat_no = $1",
                &[&payload.heat_no]
            )
            .await
            .map_err(|e|{
                dbg!(e);
                AppError::InternalServerError
            })?[0].get(0);

            let mut i: u32 = 0;
            if &payload.part_list.len() > &0 && row > 0 {
                for part in &payload.part_list {
                    let new_approved_component = ApprovedComponent {
                        approval_pk: Uuid::new_v4(),
                        heat_no: payload.heat_no.clone(),
                        grade: payload.grade.clone(),
                        section: payload.section.clone(),
                        section_type: payload.section_type.clone(),
                        approved_part: part.to_string(),
                        created_by: Some(logged_user.username.to_string()),
                        created_on: std::time::SystemTime::now(),
                        modified_by: None,
                        modified_on: None,
                    };
            
                    let insert = service.client
                    .execute(
                        "INSERT INTO intellidyn_approved_component_table (
                            approval_pk,
                            heat_no,
                            grade,
                            section,
                            section_type,
                            approved_part,
                            created_by,
                            created_on,
                            modified_by,
                            modified_on
                        ) VALUES (
                            $1,
                            $2,
                            $3,
                            $4,
                            $5,
                            $6,
                            $7,
                            $8,
                            $9,
                            $10
                        )", &[
                            &new_approved_component.approval_pk.to_string(),
                            &new_approved_component.heat_no,
                            &new_approved_component.grade,
                            &new_approved_component.section,
                            &new_approved_component.section_type,
                            &new_approved_component.approved_part,
                            match &new_approved_component.created_by {
                                Some(v) => v,
                                _ => &None::<String>
                            },
                            &new_approved_component.created_on,
                            match &new_approved_component.modified_by {
                                Some(v) => v,
                                _ => &None::<String>
                            },
                            &new_approved_component.modified_on
                        ]
                    )
                    .await
                    .map_err(|_| AppError::DataInsertionFailed)?;
        
                    if insert < 1 {
                        Err(AppError::DataInsertionFailed)
                    } else {
                        Ok(i += 1)
                    };
                }
            };
            i
        } else {
            0
        };

        Ok(Json(json!(result)))
    }

    pub async fn alter_approved_component_table(
        Extension(logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>
    ) -> Result<Json<Value>, AppError> {
        let result = service.client
        .execute("ALTER TABLE intellidyn_approved_component_table
            ADD COLUMN section INT,
            ADD COLUMN section_type TEXT;", &[]
        )
        .await
        .map_err(|e|{
            dbg!(e);
            AppError::InternalServerError
        });

        Ok(Json(json!(result)))
    }
}