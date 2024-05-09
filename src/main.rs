use std::{collections::HashMap, net::{SocketAddr, TcpListener}};

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
        
        if !url.ends_with(&config.endpoint_url) {
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
