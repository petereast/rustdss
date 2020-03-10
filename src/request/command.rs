// Parses commands
use super::deserialise::RespData;

enum Command {
    PING,
    GET(String), // Do we want to use strings or do we want to use Resp values?
    SET(String, RespData),
}

impl Command {
    pub fn from_string(input: String) -> Option<Self> {
        match input.as_str() {
            "PING" => Some(Command::PING),
            _ => None,
        }
    }
}
