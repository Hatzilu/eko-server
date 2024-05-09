use std::{fs, io::Write, net::TcpStream};

pub fn handle_response(stream: &mut TcpStream) {

    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("./public/index.html").expect("Failed to write HTML data to response");

    let response = format!("{status}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response to connection")
}