mod credentials_reader;
mod data_service;
mod clients;
mod values;
mod stock_market;

use chrono::{Utc, TimeZone};

use crate::stock_market::StockMarket;

#[tokio::main]
async fn main() {
    let start_date = Utc.with_ymd_and_hms(2025, 7, 25, 0, 0, 0).unwrap();
    let stock_market = StockMarket::new(start_date, start_date);

    println!("Hello, world!");
}
