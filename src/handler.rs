use lambda_http::{Body, Error, Request, IntoResponse, RequestExt, Response};
use crate::db_client::DynamodbClient;

pub async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let client = DynamodbClient::init().await?;

    let resp = match &client.check_tables("users").await {
        Ok(true) => "Table Exists",
        Ok(false) => "Table does not exist. Create one first.",
        Err(e) => format!("Error {:?}", e)
    };

    Ok(match event.query_string_parameters().first("first_name") {
        Some(v) => {
            format!("{}: Client invoked by {:#?}. {:?}", event.lambda_context().request_id, v, resp)
            .into_response()
            .await
        },
        _ => Response::builder()
        .status(400)
        .body("Empty first name".into())
        .expect("failed to render response"),
    })
}