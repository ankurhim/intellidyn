use intellidyn::service::DbService;
use tokio_postgres::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = DbService::new()
    .await
    .unwrap();

    let rows = client.client
    .query("SELECT $1::TEXT", &[&"hello world"])
    .await?;

    let value: &str = rows[0].get(0);
    println!("{value:?}");

    Ok(())
}