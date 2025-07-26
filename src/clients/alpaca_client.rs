use std::collections::HashMap;

use reqwest::{
    Client,
    header:: {HeaderMap, HeaderValue},
};

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
}

fn get_default_headers(api_key: &str, api_secret: &str) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    header_map.insert("APCA-API-KEY-ID", HeaderValue::from_str(api_key).expect("Valid Key"));
    header_map.insert("APCA-API-SECRET-KEY", HeaderValue::from_str(api_secret).expect("Valid Secret"));
    header_map.insert("accept", HeaderValue::from_str("application/json").expect("Valid Secret"));

    header_map
}
