use intellidyn::service::DbService;
use tokio_postgres::Error;
use intellidyn::run;

#[tokio::main]
async fn main() {
    run().await;
}