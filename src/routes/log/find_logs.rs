use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use axum::{
    Extension,
    Json,
    extract::{Query, Path}
};
use chrono::{DateTime, NaiveDate ,Local};

use serde_json::{Value, json};

use crate::routes::log::log_model::Log;
use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindLogRequest {
    pub username: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFilterRequest {
    pub start_date: String,
    pub end_date: String
}

impl FindLogRequest {
    pub async fn find_logs(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut log_vector: Vec<Log> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_log_table", &[]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            log_vector.push(Log {
                log_pk: Uuid::parse_str(row.get(1)).unwrap(),
                username: row.get(2),
                login_key: row.get(3),
                login_time: row.get(4),
                logout_time: row.get(5),
                remarks: row.get(6)
            })
        }

        Json(json!(log_vector))
    }

    pub async fn find_logs_by_username(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
    ) -> Json<Value> {
        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut log_vector: Vec<Log> = Vec::new();

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_log_table WHERE username = $1 ORDER BY login_time ASC;", &[&user]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            log_vector.push(Log {
                log_pk: Uuid::parse_str(row.get(1)).unwrap(),
                username: row.get(2),
                login_key: row.get(3),
                login_time: row.get(4),
                logout_time: row.get(5),
                remarks: row.get(6)
            })
        }

        Json(json!(log_vector))
    }

    pub async fn find_logs_by_username_filter_by_date(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<DateFilterRequest>,
    ) -> Json<Value> {
        let resp = service.client
        .query(
            "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&user, &login_key]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
                break;
            } else {
                return Json(json!("You are logged out"));
            }
        }

        let mut log_vector: Vec<Log> = Vec::new();

        let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%dT%H:%M:%S").expect("Date parsing error");
        let end_date = NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%dT%H:%M:%S").expect("Date parsing error");

        let resp = service.client
        .query(
            "SELECT * FROM mwspl_log_table WHERE username = $1 AND login_time BETWEEN $2 :: DATE AND $3 :: DATE ORDER BY login_time ASC;", &[&user, &start_date, &end_date]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            log_vector.push(Log {
                log_pk: Uuid::parse_str(row.get(1)).unwrap(),
                username: row.get(2),
                login_key: row.get(3),
                login_time: row.get(4),
                logout_time: row.get(5),
                remarks: row.get(6)
            })
        }

        Json(json!(log_vector))
    }

    pub async fn find_active_log_by_username(
        Extension(service): Extension<Arc<DbService>>,
        Query(query): Query<FindLogRequest>
    ) -> Json<Value> {

        let mut login_keys: Vec<String> = vec![];

        let resp = service.client
        .query(
            "SELECT login_key FROM mwspl_log_table WHERE username = $1 AND logout_time IS NULL;", &[&query.username]
        )
        .await
        .map_err(|e| Json(json!(e.to_string())));

        for row in resp.unwrap() {
            login_keys.push(row.get(0))
        };

        Json(json!(login_keys))
    }

    pub async fn find_count_of_active_log_by_username(
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<String>
    ) -> Json<Value> {

        match service.client
        .execute(
            "SELECT COUNT(*) FROM mwspl_log_table WHERE username = $1 AND logout_time = null", &[&payload]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
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
    //     .map_err(|e| Json(json!(FindLogResponse {
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

    //     Json(json!(FindLogResponse {
    //         data: user_vector,
    //         error: None,
    //     }))
    // }
}