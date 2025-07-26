use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use reqwest::{
    Client,
    header:: {HeaderMap, HeaderValue},
};

use serde_json::Value;

use crate::values::{TradeData, QuoteData};

pub struct AlpacaClient {
    client: Client,
    url: String,
}

impl AlpacaClient {
    pub fn new(credentials_map: &HashMap<String, String>) -> Self {
        let api_key = credentials_map.get("alpaca.markets.key").unwrap();
        let secret = credentials_map.get("alpaca.markets.secret").unwrap();

        let client = Client::builder()
            .default_headers(get_default_headers(api_key, secret))
            .build()
            .unwrap();

        AlpacaClient {
            client: client,
            url: "https://data.alpaca.markets/v2".to_owned(),
        }
    }

    pub async fn get_trades(&self, stock_name: &str, day: &str) -> Vec<TradeData> {
        let mut trades_list = Vec::new();

        for json_value in self.get_values(stock_name, "trades", day).await {
            trades_list.push(TradeData::from_value(stock_name, &json_value));
        }

        trades_list.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());

        trades_list
    }

    pub async fn get_quotes(&self, stock_name: &str, day: &str) -> Vec<QuoteData> {
        let mut quotes_list = Vec::new();

        for json_value in self.get_values(stock_name, "quotes", day).await {
            quotes_list.push(QuoteData::from_value(stock_name, &json_value));
        }

        quotes_list.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());

        quotes_list
    }

    async fn get_values(&self, stock_name: &str, query_type: &str, day: &str) -> Vec<Value> {
        let mut page_token = None;
        let mut value_list = Vec::new();

        loop {
            let json_body = self.query_trades(stock_name, query_type, page_token, day).await;
            let json_list = match json_body[query_type].as_array() {
                Some(v) => v,
                None => break,
            };

            for data_row in json_list.into_iter() {
                if data_row["x"] == "D" { continue; }

                value_list.push(data_row.clone());
            }

            page_token = match json_body["next_page_token"].as_str() {
                Some(v) => Some(v.to_owned()),
                None => break,
            };
        }

        value_list
    }

    async fn query_trades(&self, stock_name: &str, query_type: &str, page_token: Option<String>, day: &str) -> Value {
        let mut url = format!("{}/stocks/{stock_name}/{query_type}?start={day}T03:00:00-04:00&end={day}T23:00:00-04:00&limit=10000&feed=sip&sort=asc", self.url);

        if page_token.is_some() {
            url.push_str("&page_token=");
            url.push_str(&page_token.unwrap());
        }

        let body = self.send_request(&url).await;
        serde_json::from_str(&body[..]).unwrap()
    }

    async fn send_request(&self, url: &String) -> String {
        for _ in 0..12 {
            let response = self.client.get(url).send().await.unwrap();

            if response.status() == 429 {
                let _ = sleep(Duration::from_secs(5));

                continue;
            }

            return response.text().await.unwrap();
        }

        String::new()
    }
}

fn get_default_headers(api_key: &str, api_secret: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    header_map.insert("APCA-API-KEY-ID", HeaderValue::from_str(api_key).expect("Valid Key"));
    header_map.insert("APCA-API-SECRET-KEY", HeaderValue::from_str(api_secret).expect("Valid Secret"));
    header_map.insert("accept", HeaderValue::from_str("application/json").expect("Valid Secret"));

    header_map
}
