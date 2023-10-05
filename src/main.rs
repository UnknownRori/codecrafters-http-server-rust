use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn get_path<'a>(lines: &'a Vec<&'a str>) -> Option<&'a str> {
    for line in lines {
        if line.starts_with("GET") {
            return line.split(" ").nth(1);
        }
    }

    None
}

fn parse_request(stream: &mut TcpStream) -> String {
    let mut buffer: [u8; 512] = [0u8; 512];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read the stream");

    return String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
}

fn write_stream(stream: &mut TcpStream, message: &str) {
    stream
        .write(message.as_bytes())
        .expect("Failed to send a response");
}

fn handle_connection(mut stream: TcpStream) {
    let binding = parse_request(&mut stream);
    let request_lines = binding.split("\r\n").collect::<Vec<&str>>();

    let path = get_path(&request_lines).expect("Failed to get path");

    if path == "/" {
        write_stream(&mut stream, "HTTP/1.1 200 OK\r\n\r\n");
    } else {
        write_stream(&mut stream, "HTTP/1.1 404 Not Found\r\n\r\n");
    }
}

fn main() {
    let conn = "127.0.0.1:4221";
    let listener = TcpListener::bind(&conn).unwrap();

    println!("Server started at {}", &conn);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
