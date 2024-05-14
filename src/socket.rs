use std::{fs, hash::RandomState, io::Write, net::TcpStream};

use crate::http::ResponseBuilder;

pub fn handle_response(stream: &mut TcpStream, endpoint_url: &String) {

    // let status = format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Credentials: true\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Methods: GET,DELETE,PATCH,POST,PUT\r\nAccess-Control-Allow-Headers: X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version\r\n",&endpoint_url);
    let balls = ResponseBuilder::new()
        .status(&200)
        .header("Access-Control-Allow-Credentials", "true")
        .header("Access-Control-Allow-Origin","https://127.0.0.1:3000")
        .header("Access-Control-Allow-Methods", "GET,DELETE,PATCH,POST,PUT")
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Headers", "X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version");
        
        let s = balls.build();

        println!("reesponse: {s}");

        let sid = RandomState::new();
    // let data = builder.to_string();
    let contents = r#"{
    "sid": "FSDjX-test",
    "upgrades": ["websocket"],
    "pingInterval": 25000,
    "pingTimeout": 20000
}"#;

    let response = format!("{s}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to write response to connection")
}