// Parses commands
use crate::transport::RespData;

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(RespData),
    Get(String), // Do we want to use strings or do we want to use Resp values?
    Set(String, RespData),
}

impl Command {
    // Utility functions
    fn string_arg(data: Option<&RespData>) -> Option<String> {
        if let Some(inner_data) = data {
            // Clones are expensive! Be mindful of this in a performance sensitive context
            match inner_data {
                RespData::BulkStr(string) => Some(string.clone()),
                RespData::SimpleStr(string) => Some(string.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn from_resp(input: RespData) -> Result<Self, String> {
        if let RespData::List(data) = input {
            if let Some(RespData::BulkStr(cmd_string)) = data.get(0) {
                match cmd_string.as_str() {
                    "ping" => Ok(Command::Ping),
                    "echo" => {
                        if let Some(arg0) = data.get(1) {
                            Ok(Command::Echo(arg0.clone()))
                        } else {
                            Err("too few args".into())
                        }
                    }
                    "get" => {
                        if let Some(arg0) = Self::string_arg(data.get(1)) {
                            Ok(Command::Get(arg0))
                        } else {
                            Err("too few args".into())
                        }
                    }
                    "set" => {
                        if let Some(arg0) = Self::string_arg(data.get(1)) {
                            if let Some(arg1) = data.get(2) {
                                Ok(Command::Set(arg0.clone(), arg1.clone()))
                            } else {
                                Err("Not enough args".into())
                            }
                        } else {
                            Err("nope".into())
                        }
                    }
                    _ => Err("unknown command".into()),
                }
            } else {
                Err("invalid command format".into())
            }
        } else {
            Err("invalid command format".into())
        }
    }
}
