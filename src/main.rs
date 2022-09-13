mod db_client;
mod handler;

use lambda_http::{service_fn, Error, tower::ServiceBuilder};
use tower_http::cors::{ CorsLayer, Any};
use crate::handler::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    let cors_layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any);

    let service = ServiceBuilder::new()
    .layer(cors_layer)
    .service(service_fn(handler));

    lambda_http::run(service).await?;
    Ok(())
}