pub mod service;
pub mod test;
pub mod routes;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::routes::create_routes;

pub async fn run() {

    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into())
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = create_routes().await;

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}