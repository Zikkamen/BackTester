use std::{collections::HashMap, path::Path, fs::{self, File}, io::Write};

use chrono::{DateTime, Utc};

use crate::{clients::AlpacaClient, values::TradeData};

pub struct DataService {
    client: AlpacaClient,
}

impl DataService {
    pub fn new(credentials_map: &HashMap<String, String>) -> Self {
        DataService {
            client: AlpacaClient::new(credentials_map),
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
}