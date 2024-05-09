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
pub struct Request <'a>{
    pub url: &'a str,
    pub method: HttpMethod,
    pub http_version: &'a str,
    // pub stream: TcpStream,
    pub headers: HashMap<String, String>,
}

fn get_header_values(headers: &Vec<String>) -> HashMap::<String, String> {
    let mut map = HashMap::<String, String>::new();
    for val in headers.into_iter() {
        if !val.contains(": ") {
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

        
        let method = http_request.get(0).expect("balls");
        
        let headers = get_header_values(&http_request);


        let method_array = method.split(" ").collect::<Vec::<&str>>();
        let method_string = method_array.get(0).expect("Failed to get method");
        let url = method_array.get(1).expect("Failed to get method");
        let http_version = method_array.get(2).expect("Failed to get method");


        let method = HttpMethod::from(&method_string).expect("Failed toget method");
        
        return Request {
            method,
            url,
            http_version,
            headers,
        }
    }
}

// impl fmt::Display for R


