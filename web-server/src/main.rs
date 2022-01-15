use ico;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || handle_connection(&stream));
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1\r\n";
    let get_favicon = b"GET /favicon.ico HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";

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

        let image = icon_dir.entries()[0].decode().unwrap();
        let image = image.rgba_data();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {:?}\r\n\r\n{:?}",
            image.len(),
            image,
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(10));

        let response = format!("HTTP/1.1 200 OK\r\n");

        stream
            .write(response.as_bytes())
            .expect("Error writing response");
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("./public/404.html").expect("Error reading HTML file!");

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream
            .write(response.as_bytes())
            .expect("Error writing response stream!");
        stream.flush().expect("Error flushing stream!");
    }
}
