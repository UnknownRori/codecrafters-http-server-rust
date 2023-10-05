use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut _buffer: [u8; 512] = [0u8; 512];
    let _bytes_read = stream
        .read(&mut _buffer)
        .expect("Failed to read the stream");

    stream
        .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
        .expect("Failed to send response");
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
