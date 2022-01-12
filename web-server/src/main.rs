use ico;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";
    let get_favicon = b"GET /favicon.ico HTTP/1.1\r\n";

    if buffer.starts_with(get_request) {
        let contents = fs::read_to_string("./public/index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_favicon) {
        let favicon = File::open("./public/favicon.ico").unwrap();
        let icon_dir = ico::IconDir::read(favicon).unwrap();

        for entry in icon_dir.entries() {
            println!("{}x{}", entry.width(), entry.height());
        }

        let image = icon_dir.entries()[0].decode().unwrap();
        let image = image.rgba_data();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {:?}\r\n\r\n{:?}",
            image.len(),
            image,
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Unknown request!");
    }
}
