use tokio_postgres::{Client, Error};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;

pub struct DbService {
    pub client: Client
}

impl DbService {
    pub async fn new() -> Result<Self,Error> {
        let read_cert = fs::read("root.crt").unwrap();
    
        let cert = Certificate::from_pem(&read_cert).unwrap();
        
        let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();
        
        let tls_connector = MakeTlsConnector::new(connector);

        let (client, connection) = tokio_postgres::connect(
            "postgresql://intellidyn:c9eBGw_RVnp4TbCWm1z7CQ@candy-oilbird-8892.5xj.cockroachlabs.cloud:26257/intellidyn_db",
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