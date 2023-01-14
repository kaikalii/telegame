use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};

use crate::{Game, Input};

pub fn run_server<G: Game>(game: G) {
    thread::spawn(run_http_server);
    thread::spawn(move || run_frame_server(game));
    loop {
        sleep(Duration::from_secs(1));
    }
}

fn run_http_server() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || handle_http_request(stream));
    }
}

macro_rules! lprintln {
    ($($arg:tt)*) => {
        writeln!(::std::io::stdout().lock(), $($arg)*)
    };
}

fn handle_http_request(stream: io::Result<TcpStream>) {
    match handle_http_request_impl(stream) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => println!("Error: {e}"),
    }
}

fn handle_http_request_impl(stream: io::Result<TcpStream>) -> io::Result<()> {
    // Initialize the stream
    let mut stream = stream?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    lprintln!("\nGot https connection from {}", stream.peer_addr()?)?;

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
            request_lines.push(String::from_utf8_lossy(&buf).trim().to_owned());
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
            Err(e) => {
                lprintln!("unable to find {path}: {e}")?;
                ("HTTP/1.1 404 Not Found", e.to_string().into_bytes())
            }
        };
        // Write the response
        let length = contents.len();
        let mime = mime_guess::from_path(&path).first_raw().unwrap_or_default();
        stream.write_all(
            format!("{status}\r\nContent-Type: {mime}\r\nContent-Length: {length}\r\n\r\n")
                .as_bytes(),
        )?;
        stream.write_all(&contents)?;
        lprintln!("Served {path} on {}", stream.peer_addr()?)?;
    }
}

fn run_frame_server<G: Game>(mut game: G) {
    let listener = TcpListener::bind("0.0.0.0:8001").unwrap();
    for stream in listener.incoming().filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        let state = game.new_state();
        thread::scope(move |s| {
            s.spawn(move || handle_socket_request::<G>(state, stream));
        });
    }
}

fn handle_socket_request<G: Game>(state: G::State, stream: TcpStream) {
    let ip = stream.peer_addr().unwrap();
    match handle_socket_request_impl::<G>(state, stream) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::TimedOut => {
            let _ = lprintln!("{} timed out", ip);
        }
        Err(e) => {
            let _ = lprintln!("Error: {e}");
        }
    }
}
fn handle_socket_request_impl<G: Game>(
    mut state: G::State,
    mut stream: TcpStream,
) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let ip = stream.peer_addr()?;

    lprintln!("\nGot socket connection from {}", ip)?;

    let mut buffer = Vec::new();
    loop {
        buffer.clear();
        let mut reader = BufReader::new(&stream);
        let mut header_lines = Vec::new();
        let mut input_lines = Vec::new();
        for lines in [&mut header_lines, &mut input_lines].iter_mut() {
            loop {
                buffer.clear();
                reader.read_until(b'\n', &mut buffer)?;
                if buffer == b"\r\n" {
                    break;
                }
                lines.push(String::from_utf8_lossy(&buffer).trim().to_owned());
            }
        }
        let input_line = &input_lines[0];
        let (header, body) = match serde_json::from_str::<Input>(input_line) {
            Ok(input) => {
                let frame = G::make_frame(&mut state, input);
                ("HTTP/1.1 200 OK", serde_json::to_string(&frame).unwrap())
            }
            Err(e) => {
                lprintln!("Error: {e}")?;
                (
                    "HTTP/1.1 400 Bad Request",
                    serde_json::to_string(&e.to_string()).unwrap(),
                )
            }
        };
        let length = body.len();
        let content_type = "Content-Type: application/json";
        let content_length = format!("Content-Length: {length}");
        stream.write_all(
            format!("{header}\r\n{content_type}\r\n{content_length}\rn\r\n{body}").as_bytes(),
        )?;
    }
}
