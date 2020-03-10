pub mod command;

use crate::transport::RespData;
use command::Command;

pub struct Request {}

impl Request {
    pub fn handle(input: RespData) -> RespData {
        match Command::from_resp(input) {
            Ok(Command::Ping) => RespData::SimpleStr("PONG".into()),
            Ok(Command::Echo(data)) => data,
            Err(reason) => RespData::Error(reason),
            _ => RespData::Error("Unimplemented".into()),
        }
    }
}
