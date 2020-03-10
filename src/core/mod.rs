use crate::request::command::Command;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
// Exists for the lifetime of the application.
type Message = Command;
pub struct Core {
    sender: SyncSender<Message>,
}

impl Core {
    pub fn start() -> Self {
        // Could do something interesting using a threadpool - key-hash sharding for example
        println!("[core] starting core");
        let (sender, reciever) = sync_channel::<Message>(50);

        thread::spawn(move || loop {
            if let Ok(msg) = reciever.recv() {
                println!("[core] message recieved: {:?}", msg);
            } else {
                println!("[core] death");
            }
        });
        Self { sender: sender }
    }

    pub fn get_sender(&self) -> SyncSender<Message> {
        self.sender.clone()
    }
}
