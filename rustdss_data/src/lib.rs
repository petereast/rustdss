use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub enum RespData {
    Error(String),            // Errors are just text
    Number(i64),              // Numbers
    SimpleStr(String),        // Simple strings are not prefixed with length
    BulkStr(String),          // BulkStr is prefixed with it's length
    List(VecDeque<RespData>), // Lists don't have to be made up of the same type
    NullString,
}

impl RespData {
    pub fn ok() -> Self {
        Self::SimpleStr("OK".into())
    }

    pub fn nil() -> Self {
        Self::NullString
    }
    pub fn wrong_type() -> Self {
        RespData::Error("WRONGTYPE Operation against a key holding the wrong kind of value".into())
    }
}

#[cfg(test)]
mod should {
    // These are end-to-end tests
    // Showing that it can decode and re-encode data into the same thing.

    #[test]
    fn run_tests() {
        assert!(true);
    }
}

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
    Select(String),
    Lpop(Key),
    Lpush(Key, RespData),
    Rpop(Key),
    Rpush(Key, RespData),
    Llen(Key),
    Keys,
    Info,
    FlushAll,
    Dump(Key),
}
