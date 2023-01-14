use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind(("0.0.0.0", 8000)).unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || handle_request(stream));
    }
}

fn handle_request(stream: io::Result<TcpStream>) {
    match handle_request_impl(stream) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => println!("Error: {e}"),
    }
}

fn handle_request_impl(stream: io::Result<TcpStream>) -> io::Result<()> {
    // Initialize the stream
    let mut stream = stream?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    println!("\nGot connection from {}", stream.peer_addr()?);

    // Handle sequential requests
    loop {
        // Parse the request
        let mut request_lines = Vec::new();
        let mut buf = Vec::new();
        let mut read = BufReader::new(&mut stream);
        loop {
            buf.clear();
            read.read_until(b'\n', &mut buf)?;
            if buf == b"\r\n" {
                break;
            }
            request_lines.push(String::from_utf8_lossy(&buf).into_owned());
        }
        // Parse the path
        let mut path = request_lines[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_owned();
        if path == "/" {
            path = "/index.html".into();
        }
        // Read the file
        let (status, contents) = match fs::read(format!("site{path}")) {
            Ok(contents) => ("HTTP/1.1 200 OK", contents),
            Err(_) => ("HTTP/1.1 404 Not Found", "Not found!".into()),
        };
        // Write the response
        let length = contents.len();
        stream.write_all(format!("{status}\r\nContent-Length: {length}\r\n\r\n").as_bytes())?;
        stream.write_all(&contents)?;
        println!("Served {path} on {}", stream.peer_addr().unwrap());
    }
}
