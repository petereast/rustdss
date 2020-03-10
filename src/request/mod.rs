pub mod command;

use crate::core::Message;
use crate::transport::RespData;
use command::Command;
use std::sync::mpsc::{sync_channel, SyncSender};

pub struct Request {}

impl Request {
    pub fn handle(core_sender: SyncSender<Message>, input: RespData) -> RespData {
        match Command::from_resp(input) {
            Ok(Command::Ping) => RespData::SimpleStr("PONG".into()),
            Ok(Command::Echo(data)) => data,
            Ok(core_cmd) => {
                // This will dispatch a request to the core -- how to deal with the response?
                // Channels are a one way affair - maybe build a module to deal with this?
                let (return_sender, recv) = sync_channel::<RespData>(5);
                match core_sender
                    .send((core_cmd, return_sender))
                    .map_err(|_| String::from("Can't send to core"))
                    .and(
                        recv.recv()
                            .map_err(|_| String::from("Can't recv from core")),
                    ) {
                    Ok(response) => response,
                    Err(message) => RespData::Error(message),
                }
            }
            Err(reason) => RespData::Error(reason),
        }
    }
}
