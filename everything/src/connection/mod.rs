use crate::core::Message;
use crate::request::Request;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::SyncSender;
use std::thread;
use transport::RespData;

pub struct Connection {}

impl Connection {
    fn handle_incoming_stream(core_sender: SyncSender<Message>, stream: &mut TcpStream) -> () {
        // This function will create and use instances of Request
        println!("[connection], handling tcp stream from client {:?}", stream);

        let mut bytes = stream
            .try_clone()
            .unwrap()
            .bytes()
            .map(|result| result.unwrap() as char);

        let mut database_id = None;
        loop {
            if let Some(input_data) = RespData::from_char_stream(&mut bytes) {
                // Parse each request and give the parsed request to the Request module
                // Turn the bytes into a stream of chars!
                //
                // We need some state here to manage which database this connection/session is
                // talking to
                let (new_database_id, response) =
                    Request::handle(database_id.clone(), core_sender.clone(), input_data);

                database_id = new_database_id;

                stream
                    .write(response.as_string().as_bytes())
                    .expect("Can't write to socket");
            } else {
                break;
            }
        }
    }

    pub fn start(outer_core_sender: SyncSender<Message>) -> std::io::Result<Self> {
        println!("[connection] Starting to listen to connections");

        let listener = TcpListener::bind("0.0.0.0:6380")?;

        for stream in listener.incoming() {
            let core_sender = outer_core_sender.clone();
            thread::spawn(move || {
                Connection::handle_incoming_stream(core_sender.clone(), &mut stream.unwrap());
                println!("[connection] connection terminated");
            });
        }

        Ok(Self {})
    }
}
