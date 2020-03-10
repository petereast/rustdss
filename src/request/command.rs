// Parses commands
use crate::transport::RespData;

#[derive(Debug)]
pub enum Command {
    Ping,
    Get(String), // Do we want to use strings or do we want to use Resp values?
    Set(String, RespData),
}

impl Command {
    pub fn from_string(input: String) -> Option<Self> {
        match input.as_str() {
            "PING" => Some(Command::Ping),
            _ => None,
        }
    }
}
