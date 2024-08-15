use std::io::{prelude::*};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "Hello, World!")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 - Not Found")
    };

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server running on http://127.0.0.1:3000/");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
