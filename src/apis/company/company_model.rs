use lambda_http::{Request, RequestExt, Response, Body};
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use aws_sdk_dynamodb::Error;
use crate::db_client::DynamodbClient;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CompanyRequest {
    #[serde(default)]
    pub company_key: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub company_name_2: Option<String>,
    #[serde(default)]
    pub street: String,
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub view_maintenance: Option<String>,
    #[serde(default)]
    pub request: Option<String>,
    #[serde(default)]
    pub short_desc: Option<String>
}

impl CompanyRequest {
    pub async fn into_response(&self) -> Response<Body> {
        let s = serde_json::to_string_pretty(&self).unwrap();
        let body = Body::from(s);
        let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new).unwrap();

        response
    }
}

pub async fn define_company(
    request: Request
) -> Result<PutItemOutput, Error> {
    let new_company: Option<CompanyRequest> = request.payload().unwrap();

    let company_pk_av = AttributeValue::S(Uuid::new_v4().to_string());
    let company_key_av = AttributeValue::S(new_company.clone().unwrap().company_key.into());
    let company_name_av = AttributeValue::S(new_company.clone().unwrap().company_name.into());
    let company_name_2_av = match new_company.clone().unwrap().company_name_2.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().company_name_2.unwrap().into())
    };
    let street_av = AttributeValue::S(new_company.clone().unwrap().street.into());
    let postal_code_av = AttributeValue::S(new_company.clone().unwrap().postal_code.into());
    let city_av = AttributeValue::S(new_company.clone().unwrap().city.into());
    let country_av = AttributeValue::S(new_company.clone().unwrap().country.into());
    let language_av = AttributeValue::S(new_company.clone().unwrap().language.into());
    let currency_av = AttributeValue::S(new_company.clone().unwrap().currency.into());
    let view_maintenance_av = match new_company.clone().unwrap().view_maintenance.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().view_maintenance.unwrap().into())
    };
    let request_av = match new_company.clone().unwrap().request.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().request.unwrap().into())
    };
    let short_desc_av = match new_company.clone().unwrap().short_desc.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().short_desc.unwrap().into())
    };

    let client = DynamodbClient::init().await?;

    let result = client.db_client
    .put_item()
    .table_name("company")
    .item("company_pk", company_pk_av)
    .item("company_key", company_key_av)
    .item("company_name", company_name_av)
    .item("company_name_2", company_name_2_av)
    .item("street", street_av)
    .item("postal_code", postal_code_av)
    .item("city", city_av)
    .item("country", country_av)
    .item("language", language_av)
    .item("currency", currency_av)
    .item("view_maintenance", view_maintenance_av)
    .item("request", request_av)
    .item("short_desc", short_desc_av)
    .send()
    .await?;

    Ok(result)
}

pub async fn list_companies(request: Request) -> Result<Vec<CompanyRequest>, Error> {
    
    let _context = request.lambda_context();
    
    let db_client = DynamodbClient::init().await?;
    
    let companies_list: Vec<CompanyRequest> = db_client.list_items("company").await.unwrap();

    Ok(companies_list)
}