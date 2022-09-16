use lambda_http::{Body, Request, RequestExt, Response};
use crate::db_client::DynamodbClient;
use crate::apis::company::company_config;
use hyper::Method;
use crate::errors::CustomError;

pub async fn handler(event: Request) -> Result<Response<Body>, CustomError> {

    let client = DynamodbClient::init().await?;

    let resp = match &client.check_tables("users").await {
        Ok(true) => "Table Exists".to_string(),
        Ok(false) => "Table does not exist. Create one first.".to_string(),
        Err(e) => format!("Error {:?}", e)
    };

    Ok(match &event.raw_http_path()[..] {
        "company" => company_config::resolve_routes(Method::POST, "/company", event).await.unwrap(),
        _ => Response::builder()
        .status(404)
        .body(Body::from("Page not found"))
        .map_err(Box::new).unwrap(),
    })
}