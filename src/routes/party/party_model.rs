use serde::{Serialize, Deserialize };
use uuid::Uuid;

use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub party_pk: Uuid,
    pub party_id: String,
    pub party_type: String,
    pub party_name: String,
    pub party_address: String,
    pub gstn: String,
    pub contact_person: Option<String>,
    pub email_id: Option<String>,
    pub contact_no: Option<String>,
    pub created_by: Option<String>,
    pub created_on: DateTime<Local>,
    pub created_login_key: String,
    pub modified_by: Option<String>,
    pub modified_on: Option<DateTime<Local>>,
    pub modified_login_key: Option<String>,
    pub remarks: Option<String>
}