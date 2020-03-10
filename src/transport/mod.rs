pub mod deserialise;
pub mod serialise;

#[derive(Clone, Debug, PartialEq)]
pub enum RespData {
    Error(String),       // Errors are just text
    Number(i64),         // Numbers
    SimpleStr(String),   // Simple strings are not prefixed with length
    BulkStr(String),     // BulkStr is prefixed with it's length
    List(Vec<RespData>), // Lists don't have to be made up of the same type
}
