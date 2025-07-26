use chrono::{DateTime, Utc};

use crate::data_service::DataService;


pub struct StockMarket {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    data_service: DataService,
    
}

impl StockMarket {
    pub fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        StockMarket {
            start_date: start_date,
            end_date: end_date,
            data_service: DataService::new(),
        }
    }
}