use tokio_postgres::{Client, Error};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use dotenv;

#[derive(Debug)]
pub struct DbService {
    pub client: Client
}

impl DbService {
    pub async fn new() -> Result<Self,Error> {
        dotenv::dotenv().ok();

        let read_cert = fs::read("root.crt").unwrap();
    
        let cert = Certificate::from_pem(&read_cert).unwrap();
        
        let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();
        
        let tls_connector = MakeTlsConnector::new(connector);

        let connection_string = std::env::var("CONNECTION_STRING").expect("Connection String is missing!");

        let (client, connection) = tokio_postgres::connect(
            &connection_string,
            tls_connector
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(DbService {
            client
        })
    }
}