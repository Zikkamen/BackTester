use std::{path::Path, fs::{self, File}, io::Write};

use chrono::{DateTime, Utc};

use crate::{clients::AlpacaClient, values::{TradeData, QuoteData}, credentials_reader::CredentialsReader};

pub struct DataService {
    client: AlpacaClient,
}

impl DataService {
    pub fn new() -> Self {
        let credentials_reader = CredentialsReader::new("./credentials/api_keys.xml");
        let credentials_map = credentials_reader.get_credentials();

        DataService {
            client: AlpacaClient::new(&credentials_map),
        }
    }

    pub async fn get_trades(&self, stock_name: &str, day: &DateTime<Utc>) {
        let day_str = day.date_naive().to_string();
        let file_path = format!("trades/{}/{}", day_str, stock_name);
        let _ = fs::create_dir(Path::new(&format!("trades/{}", day_str)));

        let _trade_list = match fs::read_to_string(&file_path) {
            Ok(trades_str) => {
                let mut trade_list = Vec::new();

                for trade_str in trades_str.split('\n') {
                    if trade_str.len() == 0 { continue; }

                    trade_list.push(TradeData::from_string(trade_str));
                }

                trade_list
            },
            Err(_) => {
                let trade_list = self.client.get_trades(stock_name, &day_str).await;
                let mut cache_file = File::create(file_path).unwrap();
                let mut trade_concat = String::new();

                for trade in trade_list.iter() {
                    trade_concat.push_str(&trade.to_string());
                }

                let _ = cache_file.write(trade_concat.as_bytes());

                trade_list
            }
        };
    }

    pub async fn get_quotes(&self, stock_name: &str, day: &DateTime<Utc>) {
        let day_str = day.date_naive().to_string();
        let file_path = format!("quotes/{}/{}", day_str, stock_name);
        let _ = fs::create_dir(Path::new(&format!("quotes/{}", day_str)));

        let _quotes_list = match fs::read_to_string(&file_path) {
            Ok(quotes_str) => {
                let mut quotes_list = Vec::new();

                for quotes_str in quotes_str.split('\n') {
                    if quotes_str.len() == 0 { continue; }

                    quotes_list.push(QuoteData::from_string(quotes_str));
                }

                quotes_list
            },
            Err(_) => {
                let quotes_list = self.client.get_quotes(stock_name, &day_str).await;
                let mut cache_file = File::create(file_path).unwrap();
                let mut quotes_concat = String::new();

                for quotes in quotes_list.iter() {
                    quotes_concat.push_str(&quotes.to_string());
                }

                let _ = cache_file.write(quotes_concat.as_bytes());

                quotes_list
            }
        };
    }
}