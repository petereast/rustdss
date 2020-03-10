// Provide a nom parser and a serde deserialiser for RESP data
//
// This will parse the incoming commands. I imagine this file is going to get pretty big, so
// eventually we're going to have to refactor it into something smaller.

// Commands come in the form of RESP arrays of bulk strings and look something like this:
/*

C: *2\r\n
C: $4\r\n
C: LLEN\r\n
C: $6\r\n
C: mylist\r\n

S: :48293\r\n

The client request should parse into:
[ "LLEN", "mylist" ] or, in a more rusty way: LLEN("mylist")

*/

#[derive(Debug, PartialEq)]
pub enum RespData {
    Error(String),       // Errors are just text
    Number(i64),         // Numbers
    SimpleStr(String),   // Simple strings are not prefixed with length
    BulkStr(String),     // BulkStr is prefixed with it's length
    List(Vec<RespData>), // Lists don't have to be made up of the same type
}

impl RespData {
    fn parse_list(val: &mut std::str::Chars) -> Self {
        // How do we parse multi dimensional arrays?
        let count: String = val
            .by_ref()
            .scan(String::new(), |state, c| {
                if !(['\r', '\n'].contains(&c)) {
                    Some(format!("{}{}", state, c))
                } else {
                    None
                }
            })
            .collect();
        Self::Error("Fuck".into())
    }
    fn _parse_bulk_string(val: &mut std::str::Chars) -> Self {
        let x: String = val
            .by_ref()
            .scan(String::new(), |state, c| {
                if !(['\r', '\n'].contains(&c)) {
                    Some(format!("{}{}", state, c))
                } else {
                    None
                }
            })
            .collect();

        let output_string = String::from(val.as_str().trim());
        let expected_output_length: usize = x.parse().unwrap();

        if output_string.len() != expected_output_length {
            println!(
                "x: {}, actual_len: {}",
                expected_output_length,
                output_string.len()
            );
            RespData::Error(String::from("String length check failed"))
        } else {
            RespData::BulkStr(String::from(val.as_str().trim()))
        }
    }

    fn parse_bulk_string(stream: &mut std::str::Chars) -> Self {
        // A bulk string is made up of two chunks: the first is an int indicating how long the
        // string is, and the second is the string it's self
        if let Some(second_chunk) = Self::parse_chunk(stream) {
            Self::BulkStr(second_chunk.into())
        } else {
            Self::Error("Can't process bulk string".into())
        }
    }

    /// Just return the string until it reaches \r\n
    fn parse_chunk(stream: &mut std::str::Chars) -> Option<String> {
        if let Some(first) = stream.next() {
            let output: String = stream
                .scan(first, |mut state, item| {
                    if item == '\n' && *state == '\r' {
                        None
                    } else {
                        *state = item;
                        Some(item)
                    }
                })
                .collect();

            // There must be a better way of doing this!
            Some(String::from(format!("{}{}", first, output).trim()))
        } else {
            None
        }
    }

    pub fn from_char_stream(value: &mut std::str::Chars) -> Self {
        // This returns an iterator of RESP data that can be given to an interpreter
        // The data must be:
        // - Chunked - RESP data is separated by `\r\n`, we need to be able to read off chunks of
        //   this size
        // - Parsed - individual parsers must work with chunks - should be easier to work with
        // - Streamed away - the resulting RESP data should be made available as a stream (or
        //   iterator? Not sure which one is better for this purpose?)
        //  - Maybe don't return a stream and also don't consume the stream?
        let mut chunk = Self::parse_chunk(value).unwrap();
        match chunk.get(0..1) {
            Some(":") => {
                // good
                // Only uses this chunk
                match chunk.split_off(1).parse() {
                    Ok(i) => Self::Number(i),
                    Err(_) => Self::Error("Can't parse number!".into()),
                }
            }
            Some("-") => {
                // Only uses this chunk
                Self::Error(chunk.split_off(1))
            }
            Some("+") => {
                // Only uses this chunk
                Self::SimpleStr(chunk.split_off(1))
            }
            Some("$") => {
                // Uses more than one chunk - Will need to lend the stream to the parser
                // function
                // Doesn't need the current chunk, the next one will contain the entire string
                Self::parse_bulk_string(value)
            }
            Some("*") => {
                // Uses more than one chunk - Will need to lend the stream to the parser
                match chunk.split_off(1).parse::<usize>() {
                    Ok(i) => {
                        // Read the following number of VALUES, not chunks!!
                        let vals: Vec<Self> = (0..i)
                            .into_iter()
                            .map(|_| {
                                // Read the next value from the stream
                                Self::from_char_stream(value)
                            })
                            .collect();
                        Self::List(vals)
                    }
                    _ => Self::Error("Can't read list".into()),
                }
            }

            _ => {
                // bad
                Self::Error("Unknown symbol".into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_simple_strings() {
        let mut test1 = "+HELLO\r\n".chars();

        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::SimpleStr("HELLO".into())
        );

        let mut test2 = "+Hello World this Has Upper And LowerCase LEtTTers\r\n".chars();

        assert_eq!(
            RespData::from_char_stream(&mut test2),
            RespData::SimpleStr("Hello World this Has Upper And LowerCase LEtTTers".into())
        );

        let mut test3 = "+12345\r\n".chars();

        assert_eq!(
            RespData::from_char_stream(&mut test3),
            RespData::SimpleStr("12345".into())
        );
    }

    #[test]
    fn it_parses_error_strings() {
        let mut test1 = "-Error\r\n".chars();

        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::Error("Error".into())
        );
    }

    #[test]
    fn it_parses_numbers() {
        let mut test1 = ":100\r\n".chars();
        let mut test2 = ":-100\r\n".chars();
        let mut test3 = ":invalidnumber\r\n".chars();
        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::Number(100)
        );
        assert_eq!(
            RespData::from_char_stream(&mut test2),
            RespData::Number(-100)
        );
        assert_eq!(
            RespData::from_char_stream(&mut test3),
            RespData::Error("Can't parse number!".into())
        );
    }

    #[test]
    fn it_parses_bulk_strings() {
        let mut test1 = "$5\r\nHELLO\r\n".chars();
        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::BulkStr("HELLO".into())
        );

        // Police test
        let mut test2 = "$15\r\nHELLOHELLOHELLO\r\n".chars();
        assert_eq!(
            RespData::from_char_stream(&mut test2),
            RespData::BulkStr("HELLOHELLOHELLO".into())
        );
    }

    #[test]
    fn it_parses_lists() {
        let mut test1 = "*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".chars();
        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::List(vec![
                RespData::BulkStr("LLEN".into()),
                RespData::BulkStr("mylist".into()),
            ]),
        )
    }

    #[test]
    fn it_works_with_streams() {
        let mut test1 = ":123\r\n+hello\r\n-error\r\n".chars();

        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::Number(123)
        );

        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::SimpleStr("hello".into())
        );

        assert_eq!(
            RespData::from_char_stream(&mut test1),
            RespData::Error("error".into())
        );
    }
}