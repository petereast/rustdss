use crate::request::command::Command;
use crate::transport::RespData;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

// Exists for the lifetime of the application.
pub type Message = (Command, SyncSender<RespData>);
// This is the stateful part of the application
pub struct Core {
    sender: SyncSender<Message>,
}

struct CoreState {
    keyval: HashMap<String, RespData>,
}

impl Core {
    pub fn start() -> Self {
        // Could do something interesting using a threadpool - key-hash sharding for example
        println!("[core] starting core");
        let (sender, reciever) = sync_channel::<Message>(50);

        thread::spawn(move || {
            let mut state: CoreState = CoreState {
                keyval: HashMap::new(),
            };
            loop {
                if let Ok(msg) = reciever.recv() {
                    let (cmd, responder) = msg;

                    let response = match cmd {
                        Command::Set(key, value) => {
                            state.keyval.insert(key, value);
                            RespData::ok()
                        }
                        Command::Get(key) => state
                            .keyval
                            .get(&key)
                            .unwrap_or(&RespData::Error("nil".into()))
                            .clone(),
                        Command::FlushAll => {
                            state.keyval.clear();
                            RespData::ok()
                        }
                        _ => RespData::Error("Unknown core cmd".into()),
                    };

                    responder
                        .send(response)
                        .expect("[core] can't reply to messages");
                } else {
                    println!("[core] death");
                }
            }
        });
        Self { sender: sender }
    }

    pub fn get_sender(&self) -> SyncSender<Message> {
        self.sender.clone()
    }
}
