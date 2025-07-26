use chrono::DateTime;
use serde_json::Value;


#[derive(Clone)]
    pub struct QuoteData {
    pub name: String,
    pub ask_price: f64,
    pub bid_price: f64,
    pub timestamp: i64,
}

impl QuoteData {
    pub fn from_value(name: &str, json_value: &Value) -> Self {
        QuoteData { 
            name: name.to_owned(), 
            ask_price: json_value["ap"].as_f64().unwrap(), 
            bid_price: json_value["bp"].as_f64().unwrap(), 
            timestamp: DateTime::parse_from_rfc3339(json_value["t"].as_str().unwrap()).unwrap().timestamp_millis(),
        }
    }

    pub fn from_string(quotes_str: &str) -> Self {
        let split_string = quotes_str.split(";").collect::<Vec<&str>>(); 

        if split_string.len() != 4 { panic!(); }

        QuoteData { 
            name: split_string[0].to_owned(), 
            ask_price: split_string[1].parse::<f64>().unwrap(), 
            bid_price: split_string[2].parse::<f64>().unwrap(), 
            timestamp: split_string[3].parse::<i64>().unwrap(), 
        }
    }

    pub fn to_string(&self) -> String {
        format!("{};{};{};{}\n", self.name, self.ask_price, self.bid_price, self.timestamp)
    }
}