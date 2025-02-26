use dotenv::dotenv;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub email: String,
    pub api_key: String,
    pub zone: String,
    pub uri: String,
    pub record_id: String,
    pub ttl: u8
}


impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        let email = env::var("EMAIL").expect("EMAIL must be set");
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
        let zone = env::var("ZONE").expect("ZONE must be set");
        let uri = env::var("URI").expect("URI must be set");
        let record_id = env::var("RECORD_ID").expect("RECORD_ID must be set");
        let ttl: u8 = env::var("TTL").unwrap_or("60".to_string()).parse().expect("Unable to parse TTL");

        let cfg = Config {
           email: email.clone(),
           api_key: api_key.clone(),
           zone: zone.clone(),
           uri: uri.clone(),
           record_id: record_id.clone(),
           ttl: ttl.clone(),
        };

        return cfg;
    }

}

pub struct PathMethod {
    pub body: String,
    pub method: hyper::Method,
}