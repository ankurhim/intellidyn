mod db_client;

use lambda_http::{service_fn, Body, Error, IntoResponse, Request, RequestExt, Response, tower::ServiceBuilder};
use tower_http::cors::{ CorsLayer, Any};
use crate::db_client::DynamodbClient;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    let cors_layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any);

    let handler_func_closure = move |event: Request| async move {
        Result::<Response<Body>, Error>::Ok(match event.query_string_parameters().first("first_name") {
            Some(first_name) => {
                DynamodbClient::init().await?;
                format!("{}: Client invoked by {}.", event.lambda_context().request_id, first_name)
                .into_response()
                .await
            },
            _ => Response::builder()
            .status(400)
            .body("Empty first name".into())
            .expect("failed to render response"),
        })
    };

    let service = ServiceBuilder::new()
    .layer(cors_layer)
    .service(service_fn(handler_func_closure));

    lambda_http::run(service).await?;
    Ok(())
}