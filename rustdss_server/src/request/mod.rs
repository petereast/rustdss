pub mod command;

use command::ParseCommand;
use rustdss_core::Message;
use rustdss_data::Command;
use rustdss_data::RespData;
use std::sync::mpsc::{channel, Sender};

pub struct Request {}

impl Request {
    pub fn handle(
        database_id: Option<String>,
        core_sender: Sender<Message>,
        input: RespData,
    ) -> (Option<String>, RespData) {
        match Command::from_resp(input) {
            // Some commands don't even need to touch the core.
            Ok(Command::Ping) => (database_id, RespData::SimpleStr("PONG".into())),
            Ok(Command::Echo(data)) => (database_id, data),
            Ok(Command::Info) => (database_id, RespData::SimpleStr("info".into())),
            Ok(Command::Select(new_db)) => (Some(new_db), RespData::ok()),
            Ok(core_cmd) => {
                // How do we stream data from the responder?
                let (return_sender, recv) = channel::<RespData>();
                match core_sender
                    .send((
                        database_id
                            .clone()
                            .unwrap_or(crate::constants::default_database_name()),
                        core_cmd,
                        return_sender,
                    ))
                    .map_err(|_| String::from("Can't send to core"))
                    .and(
                        recv.recv()
                            .map_err(|_| String::from("Can't recv from core")),
                    ) {
                    Ok(response) => (database_id, response),
                    Err(message) => (database_id, RespData::Error(message)),
                }
            }
            Err(reason) => (database_id, RespData::Error(reason)),
        }
    }
}
