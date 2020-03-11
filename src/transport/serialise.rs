// Provide an implementation of a serde serialiser for RESP data
use super::RespData;

impl RespData {
    pub fn as_string(&self) -> String {
        match self {
            RespData::SimpleStr(data) => format!("+{}\r\n", *data),
            RespData::Number(num) => format!(":{}\r\n", *num),
            RespData::BulkStr(string) => format!("${}\r\n{}\r\n", string.len(), *string),
            RespData::Error(err_text) => format!("-{}\r\n", *err_text),
            _ => String::from("-Not serialisable"),
        }
    }

    pub fn _bytes(&self) -> std::str::Bytes {
        // This returns a stream of serialised text
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serialises_simple_strings_properly() {
        let input = RespData::SimpleStr("hello".into());

        assert_eq!(input.as_string(), "+hello\r\n",);
    }
    #[test]
    fn it_serialises_numbers_properly() {
        let input = RespData::Number(10);

        assert_eq!(input.as_string(), ":10\r\n");
    }
    #[test]
    fn it_serialises_bulk_strings_properly() {
        let input = RespData::BulkStr("hello".into());

        assert_eq!(input.as_string(), "$5\r\nhello\r\n");
    }
    #[test]
    fn it_serialises_error_strings_properly() {
        let input = RespData::Error("error".into());

        assert_eq!(input.as_string(), "-error\r\n");
    }
}
