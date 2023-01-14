use std::{
    fmt,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use sha1::{Digest, Sha1};

pub struct WebsocketServer {
    listener: TcpListener,
}

pub struct WebsocketStream {
    stream: TcpStream,
}

#[derive(Debug)]
pub enum WebsocketError {
    IO(io::Error),
    NoKey,
}

impl From<io::Error> for WebsocketError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl fmt::Display for WebsocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::NoKey => write!(f, "No key"),
        }
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    let mut s = String::new();
    for chunk in bytes.chunks(3) {
        let mut n = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (8 * (2 - i));
        }
        for i in 0..4 {
            let c = match n >> (6 * (3 - i)) & 0b111111 {
                0..=25 => (b'A' + n as u8) as char,
                26..=51 => (b'a' + (n - 26) as u8) as char,
                52..=61 => (b'0' + (n - 52) as u8) as char,
                62 => '+',
                63 => '/',
                _ => unreachable!(),
            };
            s.push(c);
        }
    }
    s
}

impl WebsocketServer {
    pub fn bind(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }
    pub fn incoming(&self) -> impl Iterator<Item = Result<WebsocketStream, WebsocketError>> + '_ {
        self.listener.incoming().map(|stream| {
            stream.map_err(Into::into).and_then(|mut stream| {
                // Read request
                let mut buffer = Vec::new();
                let mut reader = BufReader::new(&stream);
                let mut header_lines = Vec::new();
                loop {
                    reader.read_until(b'\n', &mut buffer)?;
                    if buffer == b"\r\n" {
                        break;
                    }
                    header_lines.push(String::from_utf8_lossy(&buffer).trim().to_owned());
                }
                // Send response
                let key = header_lines
                    .iter()
                    .find(|line| line.starts_with("Sec-WebSocket-Key: "))
                    .map(|line| line.split_at("Sec-WebSocket-Key: ".len()).1)
                    .ok_or(WebsocketError::NoKey)?
                    .to_owned()
                    + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
                let mut hasher = Sha1::new();
                hasher.update(key);
                let key = base64_encode(&hasher.finalize()[..]);
                let response = format!(
                    "HTTP/1.1 101 Switching Protocols\r\n\
                    Upgrade: websocket\r\n\
                    Connection: Upgrade\r\n\
                    Sec-WebSocket-Accept: {key}\r\n\
                    \r\n"
                );
                stream.write_all(response.as_bytes())?;
                Ok(WebsocketStream { stream })
            })
        })
    }
}
