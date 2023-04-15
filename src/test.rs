#[cfg(test)]

mod tests {
    use crate::service::DbService;

    #[tokio::test]
    async fn new_service_test() {
        let client = DbService::new()
        .await
        .unwrap();
    
        let rows = client.client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await.unwrap();
    
        let value: &str = rows[0].get(0);
        assert_eq!(value, "hello world");
    }
}