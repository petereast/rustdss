use crate::request::command::Command;
use crate::transport::{deserialise, RespData};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::SyncSender;

pub struct Connection {}

impl Connection {
    fn handle_incoming_stream(stream: TcpStream) -> () {
        // This function will create and use instances of Request
        println!("[connection], handling tcp stream from client {:?}", stream);

        let mut bytes = stream.bytes().map(|result| result.unwrap() as char);

        let data = RespData::from_char_stream(&mut bytes);
        // Parse each request and give the parsed request to the Request module
        // Turn the bytes into a stream of chars!
        println!("[connection] Incoming, decoded data: {:?}", data);
    }

    pub fn start(core_sender: SyncSender<Command>) -> std::io::Result<Self> {
        println!("[connection] Starting to listen to connections");

        let listener = TcpListener::bind("0.0.0.0:65533")?;

        for stream in listener.incoming() {
            Connection::handle_incoming_stream(stream?)
        }

        Ok(Self {})
    }
}
