use crate::request::command::Command;
use crate::request::Request;
use crate::transport::{deserialise, RespData};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::SyncSender;

pub struct Connection {}

impl Connection {
    fn handle_incoming_stream(core_sender: SyncSender<Command>, stream: &mut TcpStream) -> () {
        // This function will create and use instances of Request
        println!("[connection], handling tcp stream from client {:?}", stream);

        let mut bytes = stream
            .try_clone()
            .unwrap()
            .bytes()
            .map(|result| result.unwrap() as char);

        loop {
            if let Some(input_data) = RespData::from_char_stream(&mut bytes) {
                // Parse each request and give the parsed request to the Request module
                // Turn the bytes into a stream of chars!
                //
                let response = Request::handle(input_data);
                stream
                    .write(response.as_string().as_bytes())
                    .expect("Can't write to socket");
            } else {
                break;
            }
        }
    }

    pub fn start(core_sender: SyncSender<Command>) -> std::io::Result<Self> {
        println!("[connection] Starting to listen to connections");

        let listener = TcpListener::bind("0.0.0.0:6379")?;

        for stream in listener.incoming() {
            Connection::handle_incoming_stream(core_sender.clone(), &mut stream?)
        }

        Ok(Self {})
    }
}
