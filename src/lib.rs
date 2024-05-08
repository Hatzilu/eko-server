use std::env;

pub struct Config {
    pub addr: String
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let addr = env::var("SOCKET_ADDRESS").expect("Expected a socket address");

        Ok(Config {
            addr
        })
    }
}