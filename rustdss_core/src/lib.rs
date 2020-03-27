use crate::request::command::{Command, Key};
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
use transport::RespData;

mod base_logic;
mod db_logic;

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
    fn create_database(db_id: String) -> SyncSender<(Command, SyncSender<RespData>)> {
        let (db_sender, db_reciever) = sync_channel::<(Command, SyncSender<RespData>)>(50);

        thread::spawn(move || {
            let mut db_state = CoreState {
                keyval: HashMap::new(),
            };
            loop {
                if let Ok(msg) = db_reciever.recv() {
                    let (cmd, responder) = msg;
                    let response = base_logic::core_logic(&mut db_state, cmd);
                    responder
                        .send(response)
                        .expect(format!("[core::{}] can't reply to messages", db_id).as_str());
                } else {
                    println!("[core::{}] db_core died/dropped?", db_id);
                    break;
                }
            }
        });

        db_sender
    }
    pub fn start() -> Self {
        // Could do something interesting using a threadpool - key-hash sharding for example
        println!("[core] starting core");
        let (sender, reciever) = sync_channel::<Message>(50);

        // Each database get's it's own thread
        thread::spawn(move || {
            // This thread needs to keep track of all the databases available
            // each database needs it's own CoreState
            let mut databases: HashMap<DatabaseId, SyncSender<(Command, SyncSender<RespData>)>> =
                HashMap::new();

            databases.insert(
                crate::constants::default_database_name(),
                Self::create_database(crate::constants::default_database_name()),
            );

            loop {
                if let Ok(msg) = reciever.recv() {
                    let (database_id, cmd, responder) = msg;

                    if let Some(db_sender) = databases.get(&database_id) {
                        db_sender
                            .send((cmd, responder))
                            .expect("[core::router] Can't send to database");
                    } else {
                        let newdb_sender = Self::create_database(database_id.clone());
                        databases.insert(database_id, newdb_sender.clone());

                        newdb_sender
                            .send((cmd, responder))
                            .expect("[core::router] Can't send to new database");
                    }
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
