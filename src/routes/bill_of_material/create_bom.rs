use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{Extension, Json, extract::{Path}};
use serde_json::{Value, json};

use crate::service::DbService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBillOfMaterialRequest {
    pub purchase_order_no: String,
    pub po_date: String,
    pub party_id: String,
    pub po_quantity: Option<i64>,
    pub po_received_date: Option<String>,
    pub po_effective_date: Option<String>,
    pub po_status: String,
    pub po_deactive_date: Option<String>,
    pub rate: f64,
    pub item_type: String,
    pub drawing_no: String,
    pub part_name: String,
    pub part_no: String,
    pub grade: String,
    pub section: i64,
    pub section_type: String,
    pub jominy_range: Option<String>,
    pub gross_weight: f64,
    pub cut_weight: f64,
    pub manufacturing_stage: String,
    pub remarks: Option<String>
}

impl CreateBillOfMaterialRequest {
    pub async fn create_bill_of_material_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        match service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS mwspl_bill_of_material_table(
                id SERIAL NOT NULL,
                purchase_order_pk TEXT NOT NULL,
                purchase_order_no TEXT NOT NULL,
                po_date DATE NOT NULL,
                party_id TEXT NOT NULL REFERENCES mwspl_party_table(party_id) ON UPDATE CASCADE ON DELETE NO ACTION,
                po_quantity BIGINT,
                po_received_date DATE,
                po_effective_date DATE,
                po_status TEXT,
                po_deactive_date DATE,
                rate FLOAT8,
                item_type TEXT NOT NULL,
                drawing_no TEXT NOT NULL,
                part_name TEXT NOT NULL,
                part_no TEXT NOT NULL,
                grade TEXT NOT NULL,
                section BIGINT NOT NULL,
                section_type TEXT NOT NULL,
                jominy_range TEXT,
                gross_weight FLOAT8 NOT NULL,
                cut_weight FLOAT8 NOT NULL,
                manufacturing_stage TEXT NOT NULL,
                created_by TEXT NOT NULL REFERENCES mwspl_user_table(username) ON UPDATE NO ACTION ON DELETE NO ACTION,
                created_on TIMESTAMPTZ NOT NULL,
                created_login_key TEXT NOT NULL REFERENCES mwspl_log_table(login_key) ON UPDATE NO ACTION ON DELETE NO ACTION,
                modified_by TEXT REFERENCES mwspl_user_table(username) ON UPDATE CASCADE ON DELETE NO ACTION,
                modified_on TIMESTAMPTZ,
                modified_login_key TEXT REFERENCES mwspl_log_table(login_key) ON UPDATE CASCADE ON DELETE NO ACTION,
                remarks TEXT,
                UNIQUE (purchase_order_no, party_id, part_no, drawing_no, po_status)
            );",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string()))) {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn drop_bill_of_material_table(
        Extension(service): Extension<Arc<DbService>>
    ) -> Json<Value> {

        let drop_bill_of_material_table = service.client
        .execute(
            "DROP TABLE IF EXISTS mwspl_bill_of_material_table;",
            &[]
        )
        .await
        .map(|val| Json(json!(val)))
        .map_err(|e| Json(json!(e.to_string())));

        match drop_bill_of_material_table {
            Ok(v) => v,
            Err(e) => e
        }
    }

    pub async fn create_new_bill_of_material(
        Path((user, login_key)): Path<(String, String)>,
        Extension(service): Extension<Arc<DbService>>,
        Json(payload): Json<Self>,
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

        let po_date = NaiveDate::parse_from_str(&payload.po_date, "%d-%m-%Y").expect("PO Date parsing error");
        let po_received_date = match &payload.po_received_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Received Date parsing error")),
            None => None
        };
        let po_effective_date =  match &payload.po_effective_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Effective Date parsing error")),
            None => None
        };
        let po_deactive_date = match &payload.po_deactive_date {
            Some(v) => Some(NaiveDate::parse_from_str(&v, "%d-%m-%Y").expect("PO Deactive Date parsing error")),
            None => None
        };
        
        match service.client
        .execute(
            "INSERT INTO mwspl_bill_of_material_table(
                purchase_order_pk,
                purchase_order_no,
                party_id,
                po_date,
                po_quantity,
                po_received_date,
                po_effective_date,
                po_status,
                po_deactive_date,
                rate,
                item_type,
                drawing_no,
                part_name,
                part_no,
                grade,
                section,
                section_type,
                jominy_range,
                gross_weight,
                cut_weight,
                manufacturing_stage,
                created_by,
                created_on,
                created_login_key,
                modified_by,
                modified_on,
                modified_login_key,
                remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)",
            &[
                &Uuid::new_v4().to_string(),
                &payload.purchase_order_no,
                &payload.party_id,
                &po_date,
                &payload.po_quantity,
                &po_received_date,
                &po_effective_date,
                &payload.po_status,
                &po_deactive_date,
                &payload.rate,
                &payload.item_type,
                &payload.drawing_no,
                &payload.part_name,
                &payload.part_no,
                &payload.grade,
                &payload.section,
                &payload.section_type,
                &payload.jominy_range,
                &payload.gross_weight,
                &payload.cut_weight,
                &payload.manufacturing_stage,
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
        .map(|val| {println!("{}", &val); Json(json!(val))})
        .map_err(|e| {println!("{}", &e.to_string()); Json(json!(e.to_string()))}) {
            Ok(v) => v,
            Err(e) => e
        }
    }
}