mod credentials_reader;
mod data_service;
mod clients;
mod values;

use chrono::{Utc, TimeZone};
use credentials_reader::CredentialsReader;
use data_service::DataService;

#[tokio::main]
async fn main() {
    let credentials_reader = CredentialsReader::new("./credentials/api_keys.xml");
    let credentials_map = credentials_reader.get_credentials();

    let data_service = DataService::new(&credentials_map);
    data_service.get_trades("SPY", &Utc.with_ymd_and_hms(2025, 7, 25, 0, 0, 0).unwrap()).await;

    println!("Hello, world!");
}
