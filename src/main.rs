use std::net::TcpListener;

use ekoloko_server::Config;

pub mod socket;
pub mod http;

use crate::socket::handle_connection;

fn main() {
    let config = Config::build().unwrap();
    let listener = TcpListener::bind(&config.addr).expect("Failed to connect to the requested port");

    println!("Server listening on {}", &config.addr);
    for connection in listener.incoming() {

        let stream = connection.expect("Failed to get connection");
        
        handle_connection(stream, &config.endpoint_url);
    }
    println!("Hello, world!");
}
