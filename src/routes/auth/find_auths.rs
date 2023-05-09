use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::users::user_model::User;
use crate::routes::auth::auth_model::Auth;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindAuthRequest {
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Debug, Serialize)]
pub struct FindAuthResponse {
    pub data: Vec<Auth>,
    pub error: Option<String>
}

impl FindAuthRequest {
    pub async fn find_auths(
        Extension(_logged_user): Extension<Arc<User>>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let mut auth_vector: Vec<Auth> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_auth_table", &[]
        )
        .await
        .map_err(|e| Json(json!(FindAuthResponse {
            data: vec![],
            error: Some(e.to_string())
        })));

        for row in resp.unwrap() {
            auth_vector.push(Auth {
                auth_pk: Uuid::parse_str(row.get(1)).unwrap(),
                username: row.get(2),
                auth: row.get(3),
                created_by: row.get(4),
                created_on: row.get(5),
                modified_by: row.get(6),
                modified_on: row.get(7),
                remarks: row.get(8)
            })
        }

        Json(json!(auth_vector))
    }

    // pub async fn find_user_by_username(
    //     Extension(_logged_user): Extension<Arc<User>>,
    //     Extension(service): Extension<Arc<DbService>>,
    //     Query(query): Query<FindUserRequest>,
    // ) -> Json<Value> {
    //     let mut user_vector: Vec<User> = Vec::new();

    //     let resp = service.client
    //     .query(
    //         "SELECT * FROM mwspl_user_table WHERE username = $1", &[&query.username]
    //     )
    //     .await
    //     .map_err(|e| Json(json!(FindAuthResponse {
    //         data: vec![],
    //         error: Some(e.to_string())
    //     })));

    //     for row in resp.unwrap() {
    //         user_vector.push(User {
    //             user_pk: Uuid::parse_str(row.get(1)).unwrap(),
    //             full_name: row.get(2),
    //             employee_id: row.get(3),
    //             username: row.get(4),
    //             password: row.get(5),
    //             phone_no: row.get(6),
    //             created_by: row.get(7),
    //             created_on: row.get(8),
    //             modified_by: row.get(9),
    //             modified_on: row.get(10),
    //             remarks: row.get(11)
    //         })
    //     }

    //     Json(json!(FindAuthResponse {
    //         data: user_vector,
    //         error: None,
    //     }))
    // }
}