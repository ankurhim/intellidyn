use serde::{Serialize, Deserialize };
use uuid::Uuid;
use std::sync::Arc;
use chrono::{ DateTime, Local, NaiveDate };
use axum::{
    Extension,
    Json,
};

use serde_json::{Value, json};

use crate::routes::User;
use crate::service::DbService;