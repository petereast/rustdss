// Provide an implementation of a serde serialiser for RESP data
use super::RespData;

impl RespData {
    fn serialise_list(items: &Vec<RespData>) -> String {
        // NOTE: This is a naieve and probably rubbish way of serialising lists, could probably be
        // optimised at some point
        let len = items.len();

        let content: String = items.into_iter().map(|item| item.as_string()).collect();

        format!("*{}\r\n{}", len, content)
    }
    pub fn as_string(&self) -> String {
        match self {
            RespData::SimpleStr(data) => format!("+{}\r\n", *data),
            RespData::Number(num) => format!(":{}\r\n", *num),
            RespData::BulkStr(string) => format!("${}\r\n{}\r\n", string.len(), *string),
            RespData::Error(err_text) => format!("-{}\r\n", *err_text),
            RespData::List(items) => Self::serialise_list(items),
            RespData::NullString => "$-1\r\n".into(),
        }
    }

    pub fn _bytes(&self) -> std::str::Bytes {
        // This returns a stream of serialised text
        unimplemented!();
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn serialise_simple_strings_properly() {
        let input = RespData::SimpleStr("hello".into());

        assert_eq!(input.as_string(), "+hello\r\n",);
    }
    #[test]
    fn serialise_numbers_properly() {
        let input = RespData::Number(10);

        assert_eq!(input.as_string(), ":10\r\n");

        let input = RespData::Number(-10);

        assert_eq!(input.as_string(), ":-10\r\n");
    }
    #[test]
    fn serialise_bulk_strings_properly() {
        let input = RespData::BulkStr("hello".into());

        assert_eq!(input.as_string(), "$5\r\nhello\r\n");
    }
    #[test]
    fn serialise_error_strings_properly() {
        let input = RespData::Error("error".into());

        assert_eq!(input.as_string(), "-error\r\n");
    }

    #[test]
    fn serialise_simple_lists_properly() {
        let input = RespData::List(vec![
            RespData::SimpleStr("hello".into()),
            RespData::Number(100),
            RespData::Error("error".into()),
            RespData::BulkStr("hello world!".into()),
        ]);

        assert_eq!(
            input.as_string(),
            "*4\r\n+hello\r\n:100\r\n-error\r\n$12\r\nhello world!\r\n"
        );
    }

    #[test]
    fn serialise_multi_dimensional_lists_properly() {
        let input = RespData::List(vec![
            RespData::List(vec![
                RespData::SimpleStr("aaa".into()),
                RespData::SimpleStr("bbb".into()),
            ]),
            RespData::List(vec![
                RespData::SimpleStr("ccc".into()),
                RespData::SimpleStr("ddd".into()),
            ]),
            RespData::List(vec![
                RespData::SimpleStr("eee".into()),
                RespData::SimpleStr("fff".into()),
            ]),
        ]);

        assert_eq!(
            input.as_string(),
            "*3\r\n*2\r\n+aaa\r\n+bbb\r\n*2\r\n+ccc\r\n+ddd\r\n*2\r\n+eee\r\n+fff\r\n",
        )
    }
}
