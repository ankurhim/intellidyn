use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{
    Client,
    Error,
    model::{
        AttributeDefinition,
        ScalarAttributeType,
        KeySchemaElement,
        KeyType,
        ProvisionedThroughput,
    },
    output::{ CreateTableOutput, ScanOutput },
};
use serde_dynamo::aws_sdk_dynamodb_0_18::from_item;
use serde::Deserialize;
use crate::errors::CustomError;

#[derive(Debug)]
pub struct DynamodbClient {
    pub db_client: Client
}

impl DynamodbClient {
    pub async fn init() -> Result<Self, Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Ok(DynamodbClient {
            db_client: client
        })
    }

    pub async fn check_tables(&self, table: &str) -> Result<bool, Error> {
        let resp = self.db_client.list_tables().send().await?;
        let names = resp.table_names().unwrap_or_default();

        let mut table_exists = false;
        for name in names {
            println!("{:?}", &name);
            if name == table {
                table_exists = true
            }
        }

        Ok(table_exists)
    }

    pub async fn create_table(&self, table: &str, key: &str) -> Result<CreateTableOutput, Error> {
        let a_name: String = key.into();
        let table_name: String = table.into();
    
        let ad = AttributeDefinition::builder()
            .attribute_name(&a_name)
            .attribute_type(ScalarAttributeType::S)
            .build();
    
        let ks = KeySchemaElement::builder()
            .attribute_name(&a_name)
            .key_type(KeyType::Hash)
            .build();
    
        let pt = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build();
    
        let result = self.db_client
            .create_table()
            .table_name(table_name)
            .key_schema(ks)
            .attribute_definitions(ad)
            .provisioned_throughput(pt)
            .send()
            .await?;
    
        Ok(result)
    }

    pub async fn list_items<'a, T: Deserialize<'a>>(&self, table: &str) -> Result<Vec<T>, CustomError> {
        let result: ScanOutput = self.db_client
        .scan()
        .table_name(table)
        .send()
        .await?;
    
        let mut list = Vec::new();
        
        for item in result.items.unwrap() {
            let obj: T = from_item(item)?;
            list.push(obj)
        }
    
        Ok(list)
    }
}