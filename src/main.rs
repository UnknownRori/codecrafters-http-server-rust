use core::fmt;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

struct Request {
    user_agent: String,
    method: HttpMethod,
    path: String,
}

enum HttpMethod {
    Get,
    Post,
}

enum HttpCode {
    Ok200,
    Err404,
}

enum ContentType {
    TextPlain,
    None,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ContentType::TextPlain => "Content-Type: text/plain",
            ContentType::None => "",
        };

        write!(f, "{}\r\n", str)
    }
}

impl fmt::Display for HttpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            HttpCode::Ok200 => "200 Ok",
            HttpCode::Err404 => "404 Not Found",
        };

        write!(f, "{}\r\n", str)
    }
}

fn get_user_agent<'a>(lines: &'a Vec<&'a str>) -> Option<&'a str> {
    for line in lines {
        if line.starts_with("User-Agent") {
            return line.split(": ").nth(1);
        }
    }

    None
}

fn get_path<'a>(lines: &'a Vec<&'a str>) -> Option<&'a str> {
    for line in lines {
        if line.starts_with("GET") {
            return line.split(" ").nth(1);
        }
    }

    None
}

fn get_method<'a>(lines: &'a Vec<&'a str>) -> Option<HttpMethod> {
    match lines[0].split(" ").nth(0) {
        Some(val) => match val {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            _ => None,
        },
        None => None,
    }
}

fn parse_request(stream: &mut TcpStream) -> Request {
    let mut buffer: [u8; 512] = [0u8; 512];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read the stream");

    let request_header = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    let lines = request_header.split("\r\n").collect::<Vec<&str>>();

    let path = get_path(&lines).expect("Failed to get path").to_string();
    let user_agent = get_user_agent(&lines)
        .expect("Failed to get user agent")
        .to_string();
    let method = get_method(&lines).expect("Failed to get method");

    Request {
        path,
        user_agent,
        method,
    }
}

fn respond(http: HttpCode, content_type: ContentType, message: &str) -> String {
    let mut response = "HTTP/1.1 ".to_owned();
    response.push_str(http.to_string().as_str());

    match content_type {
        ContentType::TextPlain => response.push_str(content_type.to_string().as_str()),
        ContentType::None => (),
    };

    if !message.is_empty() {
        let length_message = message.len().to_string();
        response.push_str(format!("Content-Length: {}\r\n\r\n", length_message).as_str());
        response.push_str(message);
    }

    response.push_str("\r\n");

    response
}

fn write_stream(stream: &mut TcpStream, message: &str) {
    println!("send stream : {:#?}", message);

    stream
        .write(message.as_bytes())
        .expect("Failed to send a response");
}

fn handle_connection(mut stream: TcpStream) {
    let request = parse_request(&mut stream);

    println!("Got path: {}", request.path);
    if request.path.starts_with("/echo") {
        let content = request.path.replace("/echo/", "");
        let response = respond(HttpCode::Ok200, ContentType::TextPlain, &content);
        write_stream(&mut stream, &response);
    } else if request.path == "/user-agent" {
        let response = respond(
            HttpCode::Ok200,
            ContentType::TextPlain,
            request.user_agent.as_str(),
        );
        write_stream(&mut stream, &response);
    } else if request.path == "/" {
        let response = respond(HttpCode::Ok200, ContentType::None, "");
        write_stream(&mut stream, &response);
    } else {
        let response = respond(HttpCode::Err404, ContentType::None, "");
        write_stream(&mut stream, &response);
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
