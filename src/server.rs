use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};

use serde::Serialize;
use websocket::{
    server::upgrade::WsUpgrade,
    sync::{server::upgrade::Buffer, Server},
    OwnedMessage, WebSocketResult,
};

use crate::{Frame, Input};

type MakeFrame = fn(Input) -> Frame;

pub fn run_server(make_frame: MakeFrame) {
    thread::spawn(run_http_server);
    thread::spawn(move || run_socket_server(make_frame));
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

fn run_socket_server(make_frame: MakeFrame) {
    let server = Server::bind("0.0.0.0:8001").unwrap();
    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || handle_socket_request(request, make_frame).unwrap());
    }
}

macro_rules! lprintln {
    ($($arg:tt)*) => {
        writeln!(::std::io::stdout().lock(), $($arg)*)?
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
    lprintln!("\nGot https connection from {}", stream.peer_addr()?);

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
        if path == "/getframe" {
            // Frame path
            stream.write_all("HTTP/1.1 200 OK".as_bytes())?;
            lprintln!("Served frame on {}", stream.peer_addr()?)
        } else {
            // Site paths
            if path == "/" {
                path = "/index.html".into();
            }
            // Read the file
            let (status, contents) = match fs::read(format!("site{path}")) {
                Ok(contents) => ("HTTP/1.1 200 OK", contents),
                Err(e) => {
                    lprintln!("unable to find {path}: {e}");
                    ("HTTP/1.1 404 Not Found", e.to_string().into_bytes())
                }
            };
            // Write the response
            let length = contents.len();
            stream.write_all(format!("{status}\r\nContent-Length: {length}\r\n\r\n").as_bytes())?;
            stream.write_all(&contents)?;
            lprintln!("Served {path} on {}", stream.peer_addr()?);
        }
    }
}

#[derive(Default, Serialize)]
struct Response {
    success: bool,
    frame: Option<Frame>,
    error: Option<String>,
}

fn handle_socket_request(
    request: WsUpgrade<TcpStream, Option<Buffer>>,
    make_frame: MakeFrame,
) -> WebSocketResult<()> {
    // if !request.protocols().contains(&"rust-websocket".to_string()) {
    //     request.reject().unwrap();
    //     return Ok(());
    // }
    let client = request.use_protocol("rust-websocket").accept().unwrap();
    let ip = client.peer_addr().unwrap();

    lprintln!("\nGot socket connection from {}", ip);

    let (mut receiver, mut sender) = client.split()?;

    for message in receiver.incoming_messages() {
        let message = message?;
        match message {
            OwnedMessage::Text(message) => {
                let resp = match serde_json::from_str::<Input>(&message) {
                    Ok(input) => Response {
                        success: true,
                        frame: Some(make_frame(input)),
                        ..Default::default()
                    },
                    Err(e) => Response {
                        success: false,
                        error: Some(format!("Unable to parse input json: {e}")),
                        ..Default::default()
                    },
                };
                sender.send_message(&OwnedMessage::Text(serde_json::to_string(&resp).unwrap()))?;
            }
            OwnedMessage::Ping(ping) => sender.send_message(&OwnedMessage::Pong(ping))?,
            OwnedMessage::Pong(_) => (),
            OwnedMessage::Close(_) => {
                let message = OwnedMessage::Close(None);
                sender.send_message(&message)?;
                lprintln!("Client {} disconnected", ip);
                return Ok(());
            }
            OwnedMessage::Binary(_) => lprintln!("Unexpected binary message from {ip}"),
        }
    }
    Ok(())
}