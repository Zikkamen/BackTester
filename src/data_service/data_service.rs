use std::collections::HashMap;

use crate::clients::AlpacaClient;

pub struct DataService {
    client: AlpacaClient,
}

impl DataService {
    pub fn new(credentials_map: &HashMap<String, String>) -> Self {
        DataService {
            client: AlpacaClient::new(credentials_map),
        }
    }
}