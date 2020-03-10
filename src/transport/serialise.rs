// Provide an implementation of a serde serialiser for RESP data
use super::RespData;

impl RespData {
    pub fn as_string(&self) -> String {
        match self {
            RespData::SimpleStr(data) => format!("+{}\r\n", *data),
            RespData::Number(num) => format!(":{}\r\n", *num),
            RespData::BulkStr(string) => format!("${}\r\n{}\r\n", string.len(), *string),
            RespData::Error(err_text) => format!("-{}\r\n", *err_text),
            _ => String::from("+OK"),
        }
    }
}
