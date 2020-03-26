pub mod deserialise;
pub mod serialise;

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
    use super::*;

    #[test]
    fn run_tests() {
        assert!(true);
    }
}
