pub mod service;
pub mod test;
pub mod routes;

use crate::routes::create_routes;

pub async fn run() {

    let app = create_routes().await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}