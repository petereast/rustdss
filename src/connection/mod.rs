use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct Connection {}

impl Connection {
    fn handle_incoming_stream(stream: TcpStream) -> () {
        // This function will create and use instances of Request
        println!("[connection], handling tcp stream from client {:?}", stream);

        let bytes = stream.bytes();
        // Parse each request and give the parsed request to the Request module
    }
    pub fn start() -> std::io::Result<Self> {
        println!("[connection] Starting to listen to connections");

        let listener = TcpListener::bind("0.0.0.0:65533")?;

        for stream in listener.incoming() {
            Connection::handle_incoming_stream(stream?)
        }

        Ok(Self {})
    }
}
