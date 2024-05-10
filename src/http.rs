use core::fmt;
use std::{collections::HashMap, io::{BufRead, BufReader}, net::{SocketAddr, TcpStream}};



#[derive(Debug)]
pub struct Request{
    // pub stream: TcpStream,
    pub headers: HashMap<String, String>,
    pub peer_addr: SocketAddr,
}

fn get_header_values(headers: &Vec<String>) -> HashMap::<String, String> {
    let mut map = HashMap::<String, String>::new();
    for val in headers.into_iter() {
        if !val.contains(": ") {
            let iter = val.split(" ").collect::<Vec::<&str>>();
            let method = iter.get(0).expect("Failed to get method").to_string();
            let url = iter.get(1).expect("failed to get url").to_string();
            let http_version = iter.get(2).expect("failed to get HTTP version").to_string();

            map.insert("method".to_string(), method);
            map.insert("url".to_string(), url);
            map.insert("http_version".to_string(), http_version);
            continue;
        }
        let split_header = val.split(": ").collect::<Vec::<&str>>();

        let key = split_header.get(0).expect("Failed to get header key");
        let val = split_header.get(1).expect("Failed to get header value");
        map.insert(key.to_string(), val.to_string());
    }
    return map;
}

fn request_stream_to_vec(mut stream: &TcpStream) -> Vec<String>{
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("failed to map request"))
        .take_while(|line| !line.is_empty())
        .collect();
    return http_request;
}

impl Request {
    pub fn new(stream: &TcpStream) -> Request {
        let http_request = request_stream_to_vec(stream);
        
        let headers = get_header_values(&http_request);
        
        let peer_addr = stream.peer_addr().expect("Failed to get peer IP address");
        return Request {
            headers,
            peer_addr
        }
    }
    pub fn url(&self) -> Option<&String> {
        self.headers.get("url")
    }
    pub fn method(&self) -> Option<&String> {
        self.headers.get("method")
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request {{\r\n").expect("Failed to display Request headers");
        for (key, value) in self.headers.iter() {
            write!(f, "\t{}: {} \r\n", key, value).expect("Failed to display Request headers");
        }
        write!(f, "}}").expect("Failed to display Request headers");
        Ok(())
    }
}

pub struct ResponseBuilder {
    response: String
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder{
            response: String::from("HTTP/1.1")
        }
    }
    pub fn build(self) -> String {
        self.response
    }
    pub fn status(mut self, status: &u16) -> ResponseBuilder {
        let status_string = match status {
            200 => "200 OK",
            400 => "400 Bad Request",
            426 => "426 Upgrade Required",
            _ => todo!("TODO handle edge cases with invalid status codes"),
        };

        if !self.response.ends_with(" ") {
            self.response.push_str(" ");
        }
        self.response.push_str(&status_string);
        self.response.push_str("\r\n");
        
        return self
    }
    pub fn header(mut self, header: &str, value: &str) -> ResponseBuilder {
        let new_header = format!("{}: {}",&header, &value);
        if !self.response.ends_with(" ") {
            self.response.push_str(" ");
        }
        self.response.push_str(&new_header);
        self.response.push_str("\r\n");

        return self
    }
}