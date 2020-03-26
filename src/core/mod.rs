use crate::request::command::{Command, Key};
use crate::transport::RespData;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

mod base_logic;
mod key_val;
mod number;

pub type DatabaseId = String;

pub type Message = (DatabaseId, Command, SyncSender<RespData>);
// This is the stateful part of the application
pub struct Core {
    sender: SyncSender<Message>,
}

pub struct CoreState {
    keyval: HashMap<Key, RespData>,
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
                    let (database_id, cmd, responder) = msg;

                    let response = base_logic::core_logic(&mut state, cmd);
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
