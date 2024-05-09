use std::{fs, io::Write, net::TcpStream};

use crate::http::Request;

pub fn handle_connection(stream: &mut TcpStream, endpoint_url: &String) {

    let req = Request::new(stream);

    let url = req.url().expect("Failed to get request URL");
    
    if !url.ends_with(endpoint_url) {
        return;
    }

    let method = req.method().expect("failed to get method for print");

    println!("{} {}", &method, &url);


    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("./public/index.html").expect("Failed to write HTML data to response");

    let response = format!("{status}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response to connection")
}