use std::io::Read;
use std::net::TcpListener;

use ekoloko_server::Config;
fn main() {
    let config = Config::build().unwrap();
    let listener = TcpListener::bind(config.addr).expect("Expected a TcpListener");

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("Connection failed: {}",&e),
            Ok(mut tcp_stream) => {
                println!("new stream");
                let mut buf = String::with_capacity(512);
                
                tcp_stream.read_to_string(&mut buf).expect("expected to read TCP stream to string");
                println!("data: {}",&buf);
            }
        }
    }
    println!("Hello, world!");
}
