// Parses commands
use crate::transport::RespData;

pub type Key = String;
pub type Number = i64;
#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(RespData),
    Get(Key), // Do we want to use strings or do we want to use Resp values?
    Set(Key, RespData),
    Incr(Key, Option<Number>),
    Decr(Key, Option<Number>),
    Info,
    FlushAll,
}

impl Command {
    // Utility functions
    fn string_arg(data: Option<RespData>) -> Option<String> {
        data.and_then(|inner_data| match inner_data {
            RespData::BulkStr(string) => Some(string),
            RespData::SimpleStr(string) => Some(string),
            _ => None,
        })
    }

    fn numerical_arg(data: Option<RespData>) -> Option<Number> {
        data.clone() // ew gross
            .and_then(|val| match val {
                RespData::Number(i) => Some(i),
                // Try and coerce the string arg
                _ => None,
            })
            .or_else(|| {
                let number: Option<i64> = Self::string_arg(data).map(|s| s.parse().ok()).flatten();
                number
            })
    }

    pub fn from_resp(input: RespData) -> Result<Self, String> {
        if let RespData::List(data) = input {
            let mut data = data.into_iter();
            if let Some(RespData::BulkStr(cmd_string)) = data.next() {
                match cmd_string.to_lowercase().as_str() {
                    "ping" => Ok(Command::Ping),
                    "echo" => {
                        if let Some(arg0) = data.next() {
                            Ok(Command::Echo(arg0.clone()))
                        } else {
                            Err("too few args".into())
                        }
                    }
                    "get" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            Ok(Command::Get(arg0))
                        } else {
                            Err("too few args".into())
                        }
                    }
                    "set" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            if let Some(arg1) = data.next() {
                                Ok(Command::Set(arg0, arg1.clone()))
                            } else {
                                Err("Not enough args".into())
                            }
                        } else {
                            Err("nope".into())
                        }
                    }
                    "flushall" => Ok(Command::FlushAll),
                    "decr" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            Ok(Command::Decr(arg0, None))
                        } else {
                            Err("Not enough args".into())
                        }
                    }
                    "decrby" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            Ok(Command::Decr(arg0, Self::numerical_arg(data.next())))
                        } else {
                            Err("Not enough args".into())
                        }
                    }
                    "incr" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            Ok(Command::Incr(arg0, None))
                        } else {
                            Err("Not enough args".into())
                        }
                    }
                    "incrby" => {
                        if let Some(arg0) = Self::string_arg(data.next()) {
                            Ok(Command::Incr(arg0, Self::numerical_arg(data.next())))
                        } else {
                            Err("Not enough args".into())
                        }
                    }
                    "info" => Ok(Command::Info),
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
