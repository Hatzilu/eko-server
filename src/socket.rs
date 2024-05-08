use std::{fs, io::{BufRead, BufReader, Write}, net::TcpStream};


pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("failed to map request"))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("./public/index.html").expect("Failed to write HTML data to response");

    let response = format!("{status}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response to connection")
}