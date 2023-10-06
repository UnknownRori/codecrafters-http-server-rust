use core::fmt;
use std::env;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

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
    OctetStream,
    None,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ContentType::TextPlain => "Content-Type: text/plain",
            ContentType::OctetStream => "Content-Type: application/octet-stream",
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

async fn parse_request(stream: &mut TcpStream) -> Request {
    let mut buffer: [u8; 512] = [0u8; 512];
    let bytes_read = stream
        .read(&mut buffer)
        .await
        .expect("Failed to read the stream");

    let request_header = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    let lines = request_header.split("\r\n").collect::<Vec<&str>>();

    let path = get_path(&lines).expect("Failed to get path").to_string();
    let user_agent = get_user_agent(&lines).unwrap_or("").to_string();
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
        ContentType::None => (),
        _ => response.push_str(content_type.to_string().as_str()),
    };

    if !message.is_empty() {
        let length_message = message.len().to_string();
        response.push_str(format!("Content-Length: {}\r\n\r\n", length_message).as_str());
        response.push_str(message);
    }

    response.push_str("\r\n");

    response
}

async fn write_stream(stream: &mut TcpStream, message: &str) {
    println!("send stream : {:#?}", message);

    stream
        .write(message.as_bytes())
        .await
        .expect("Failed to send a response");
}

async fn handle_connection(mut stream: TcpStream, filedir: &str) {
    let request = parse_request(&mut stream).await;

    println!("Got path: {}", request.path);
    if request.path.starts_with("/echo") {
        let content = request.path.replace("/echo/", "");
        let response = respond(HttpCode::Ok200, ContentType::TextPlain, &content);
        write_stream(&mut stream, &response).await;
    } else if request.path == "/user-agent" {
        let response = respond(
            HttpCode::Ok200,
            ContentType::TextPlain,
            request.user_agent.as_str(),
        );
        write_stream(&mut stream, &response).await;
    } else if request.path.starts_with("/files") {
        let content = request.path.replace("/files/", "");
        let path = format!("{}/{}", filedir, content);
        let file = File::open(path).await;
        match file {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .await
                    .expect("Failed to read a file");
                let response = respond(HttpCode::Ok200, ContentType::OctetStream, &buffer);
                write_stream(&mut stream, &response).await;
            }
            Err(_) => {
                let response = respond(HttpCode::Err404, ContentType::None, "");
                write_stream(&mut stream, &response).await;
            }
        }
    } else if request.path == "/" {
        let response = respond(HttpCode::Ok200, ContentType::None, "");
        write_stream(&mut stream, &response).await;
    } else {
        let response = respond(HttpCode::Err404, ContentType::None, "");
        write_stream(&mut stream, &response).await;
    }
}

#[tokio::main]
async fn main() {
    let conn = "127.0.0.1:4221";
    let listener = TcpListener::bind(&conn).await.unwrap();
    let args = env::args().collect::<Vec<String>>();
    println!("{:#?}", args);

    let directory = if args.len() > 2 && args[1] == "--directory" {
        Arc::new(Mutex::new(args[2].to_owned()))
    } else {
        Arc::new(Mutex::new(String::new()))
    };

    println!("Server started at {}", &conn);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let directory_clone = Arc::clone(&directory);
        tokio::spawn(async move {
            println!("accepted new connection");
            let directory = directory_clone.lock().await;
            handle_connection(stream, &directory).await;
        });
    }
}
