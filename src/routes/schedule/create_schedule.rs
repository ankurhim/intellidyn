use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::{ DateTime, NaiveDate, Local, Month };

use crate::service::DbService;

#[derive(Debug, Clone Serialize, Deserialize)]
pub struct CreateScheduleRequest {
    pub schedule_month: Month,
    pub schedule_year: i64,
    pub drawing_no: String,
    pub similar_part_no: Option<String>,
    pub customer_plant: String,
    pub supplier_plant: String,
    pub most_critical_commitment_date: Option<NaiveDate>,
    pub critical_qty: Option<i64>,
    pub critical_commitment_date: Option<NaiveDate>,
    pub normal_qty: Option<i64>,
    pub normal_commitment_date: Option<NaiveDate>,
    pub total_forging_qty: i64,
    pub recv_till: Option<i64>,
    pub balance_qty: i64,
    pub remarks: Option<String>
}

impl CreateScheduleRequest {
    pub async fn create_schedule_table(
        Extension(service): Extension<DbService>
    ) -> Json<Value> {
        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_schedule_table (
                id SERIAL NOT NULL,
                schedule_pk TEXT NOT NULL,
                schedule_month TEXT NOT NULL,
                schedule_year TEXT NOT NULL,
                drawing_no TEXT NOT NULL,
                similar_part_no TEXT NOT NULL,
                customer_plant TEXT NOT NULL,
                supplier_plant TEXT NOT NULL,
                most_critical_qty BIGINT,
                most_critical_commitment_date DATE,
                critical_qty BIGINT,
                critical_commitment_date DATE,
                normal_qty BIGINT,
                normal_commitment_date DATE,
                total_forging_qty BIGINT NOT NULL,
                recv_till BIGINT,
                balance_qty BIGINT NOT NULL,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (schedule_month, schedule_year, drawing_no)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_schedule_table(
        Extension(service): Extension<DbService>
    ) -> Json<Value> {
        match service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_schedule_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn truncate_schedule_table(
        Extension(service): Extension<DbService>
    ) -> Json<Value> {
        match service.client
        .execute(
            "TRUNCATE TABLE mwspl_schedule_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_schedule(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>
    ) -> Json<Value> {
        match service.client
        .execute(
            "INSERT INTO mwspl_schedule_table (
                schedule_pk,
                schedule_month,
                schedule_year,
                drawing_no,
                similar_part_no,
                customer_plant,
                supplier_plant,
                most_critical_qty,
                most_critical_commitment_date,
                critical_qty,
                critical_commitment_date,
                normal_qty,
                normal_commitment_date,
                total_forging_qty,
                recv_till,
                balance_qty,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)",
            &[
                Uuid::new_v4().to_string(),
                &payload.schedule_month,
                &payload.schedule_year,
                &payload.drawing_no,
                &payload.similar_part_no,
                &payload.customer_plant,
                &payload.supplier_plant,
                &payload.most_critical_commitment_date,
                &payload.critical_qty,
                &payload.critical_commitment_date,
                &payload.normal_qty,
                &payload.normal_commitment_date,
                &payload.total_forging_qty,
                &payload.recv_till,
                &payload.balance_qty,
                &user,
                &Local::now(),
                &login_key,
                &None::<String>,
                &None::<DateTime<Local>>,
                &None::<String>,
                &payload.remarks
            ]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|err| Json(json!(err.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}