pub mod service;
pub mod test;
pub mod routes;

use crate::routes::create_routes;

pub async fn run() {

    let app = create_routes().await;

    axum::Server::bind(&"192.168.0.118:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}