use std::env;
use dotenv::dotenv;

pub struct Config {
    pub addr: String,
    pub endpoint_url: String
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        dotenv().ok();
        let addr = env::var("SOCKET_ADDRESS").expect("Expected a socket address");
        let endpoint_url = env::var("ENDPOINT_URL").expect("Expected an endpoint URL");

        Ok(Config {
            addr,
            endpoint_url
        })
    }
}



