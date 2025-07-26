use chrono::DateTime;
use serde_json::Value;

#[derive(Debug)]
pub struct TradeData {
    pub name: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: i64,
}

impl TradeData {
    pub fn from_value(name: &str, json_value: &Value) -> Self {
        TradeData { 
            name: name.to_owned(), 
            price: json_value["p"].as_f64().unwrap(), 
            volume: json_value["s"].as_f64().unwrap(), 
            timestamp: DateTime::parse_from_rfc3339(json_value["t"].as_str().unwrap()).unwrap().timestamp_millis(),
        }
    }

    pub fn from_string(trade_str: &str) -> Self {
        let split_string = trade_str.split(";").collect::<Vec<&str>>(); 

        if split_string.len() != 4 { panic!(); }

        TradeData { 
            name: split_string[0].to_owned(), 
            price: split_string[1].parse::<f64>().unwrap(), 
            volume: split_string[2].parse::<f64>().unwrap(), 
            timestamp: split_string[3].parse::<i64>().unwrap(), 
        }
    }

    pub fn to_string(&self) -> String {
        format!("{};{};{};{}\n", self.name, self.price, self.volume, self.timestamp)
    }
}
