use std::{collections::HashMap, io::Write, net::{SocketAddr, TcpListener}};

use ekoloko_server::Config;

pub mod socket;
pub mod http;

use crate::{http::Request, socket::handle_response};

fn main() {
    let config = Config::build().unwrap();
    let listener = TcpListener::bind(&config.addr).expect("Failed to connect to the requested port");


    let mut socket_map: HashMap<SocketAddr, Request> = HashMap::new();

    println!("Server listening on {}", &config.addr);
    for connection in listener.incoming() {

        let mut stream = connection.expect("Failed to get connection");
        
        let req = Request::new(&stream);

        let url = req.url().expect("Failed to get request URL");
        
        println!("{}",&req);
        
        // ignore requests not directed to /socket
        if !url.ends_with(&config.endpoint_url) {
            continue;
        }

        // ignore non-websocket requests
        if !req.headers.contains_key("Upgrade") {
            let response = "HTTP/1.1 426 Upgrade Required\r\n\r\nUpgrade: websocket\r\n\r\nConnection: Upgrade";
            stream.write_all(response.as_bytes()).expect("Failed to write 426 response to connection");
            continue;
        }

        // verify the upgrade header contains "websocket" value
        let upgrade_header_value = req.headers.get("Upgrade").expect("Failed to get Upgrade header").as_str();
        if upgrade_header_value.to_lowercase() != "websocket" {
            let response = "HTTP/1.1 426 Upgrade Required\r\n\r\nUpgrade: websocket\r\n\r\nConnection: Upgrade";
            stream.write_all(response.as_bytes()).expect("Failed to write 426 response to connection");
            continue;
        }
    
        let method = req.method().expect("failed to get method for print");
    
        println!("{} {}", &method, &url);

        handle_response(&mut stream);

        println!("added/updated socket map at {} ",&req.peer_addr);
        socket_map.insert(req.peer_addr, req);

        for k in  socket_map.keys() {
            println!("{}",&k);
        }
    }
    println!("Hello, world!");
}
