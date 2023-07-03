use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local };
use axum::{Extension, Json, extract::{Path}, http::StatusCode};
use serde_json::{Value, json};
use csv::Writer;

use crate::service::DbService;
use crate::routes::crud::response::Response;


pub async fn create<T: std::fmt::Debug +
Clone + 
Serialize + 
for<'a>Deserialize<'a> + 
std::marker::Sync + 
tokio_postgres::types::ToSql>
(
    Path((username, login_key)): Path<(String, String)>,
    Extension(service): Extension<Arc<DbService>>,
    Json(payload): Json<T>,
    query_string: String
) -> Response {
    let resp = service.client
    .query(
        "SELECT logout_time FROM mwspl_log_table WHERE username = $1 AND login_key = $2;", &[&username, &login_key]
    )
    .await
    .map_err(|e| Json(json!(e.to_string())));

    for row in resp.unwrap() {
        if row.get::<usize, Option<DateTime<Local>>>(0) == None::<DateTime<Local>> {
            break;
        } else {
            return Response {
                status_code: StatusCode::UNAUTHORIZED,
                data: None,
                error: Some("Unauthorized Access".to_string()),
                success: false
            };
        }
    };

    match service.client
    .execute(
        &query_string,
        &[
            &Uuid::new_v4().to_string(),
            &convert_to_csv(payload),       //csv required here
            &None::<String>,
            &username,
            &Local::now(),
            &login_key,
            &None::<String>,
            &None::<DateTime<Local>>,
            &None::<String>
        ]
    )
    .await
    .map(|v| Response{
        status_code: StatusCode::OK,
        data: Some(v.to_string()),
        error: None,
        success: true
    })
    .map_err(|e| Response{
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        data: None,
        error: Some(e.to_string()),
        success: false
    }) {
        Ok(v) => v,
        Err(e) => e
    }
}

fn convert_to_csv<T: std::fmt::Debug +
Clone + 
Serialize + 
for<'a>Deserialize<'a> + 
std::marker::Sync + 
tokio_postgres::types::ToSql>(t: T) -> String {
    let mut wtr = Writer::from_writer(vec![]);

    wtr.serialize(t).unwrap();

    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}