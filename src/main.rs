mod credentials_reader;
mod data_service;
mod clients;

use std::collections::HashMap;

use credentials_reader::CredentialsReader;
use data_service::DataService;

#[tokio::main]
async fn main() {
    let credentials_reader = CredentialsReader::new("./credentials/api_keys.xml");
    let credentials_map = credentials_reader.get_credentials();

    let data_service = DataService::new(&credentials_map);

    println!("Hello, world!");
}
