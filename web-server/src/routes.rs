pub fn route_handler(request: [u8; 1024]) -> String {
    let first_line = request
        .split(|line| line == "\r\n".as_bytes)
        .into_iter()
        .next()
        .expect("Error reading first line of request!");

    let (status_code, path) = match first_line {
        b"GET / HTTP/1.1\r\n" => ("200", "./public/index.html"),
    };
}
