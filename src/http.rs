use core::fmt;
use std::{collections::HashMap, io::{BufRead, BufReader}, net::TcpStream};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH
}

#[derive(Debug)]
pub struct HttpMethodError;

impl fmt::Display for HttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid method")
    }
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
        }
    }

    pub fn from(s: &str) -> Result<HttpMethod, HttpMethodError> {

        let enum_value = match s {
             "GET" => Ok(HttpMethod::GET),
             "POST" => Ok(HttpMethod::POST),
             "PUT" => Ok(HttpMethod::PUT),
             "PATCH" => Ok(HttpMethod::PATCH),
             &_ => Err(HttpMethodError),
        };

        return enum_value;
    }
}

#[derive(Debug)]
pub struct Request{
    // pub stream: TcpStream,
    pub headers: HashMap<String, String>,
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
    pub fn new(stream: TcpStream) -> Request {

        let http_request = request_stream_to_vec(&stream);

        
        
        let headers = get_header_values(&http_request);


        
        return Request {
            headers,
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
        for (key, value) in self.headers.iter() {
            write!(f, "Key: {} | Value: {} \r\n", key, value).expect("Failed to display Request headers");
        }
        Ok(())
    }
}


