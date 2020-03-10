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
    fn parse_bulk_string(val: &mut std::str::Chars) -> Self {
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
                Self::Error("Unimplemented".into())
            }
            Some("*") => {
                // Uses more than one chunk - Will need to lend the stream to the parser
                Self::Error("Unimplemented".into())
            }

            _ => {
                // bad
                Self::Error("Unknown symbol".into())
            }
        }
    }
    pub fn from_string(value: String) -> Result<Self, String> {
        // This needs to be from a stream
        let mut chars = value.chars();

        Ok(match chars.next().unwrap_or('?') {
            '-' => RespData::Error(String::from(chars.as_str().trim())),
            ':' => RespData::Number(
                String::from(chars.as_str())
                    .trim()
                    .parse()
                    .map_err(|_| "Can't parse integer")?,
            ),
            '+' => RespData::SimpleStr(String::from(chars.as_str().trim())),
            '$' => RespData::parse_bulk_string(&mut chars),
            '*' => unimplemented!(), // Parse list of RESP values
            _ => RespData::Error("Unknown leading character".into()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_simple_strings() {
        let test1 = String::from("+HELLO\r\n");

        assert_eq!(
            RespData::from_string(test1),
            Ok(RespData::SimpleStr("HELLO".into()))
        );

        let test2 = String::from("+Hello World this Has Upper And LowerCase LEtTTers\r\n");

        assert_eq!(
            RespData::from_string(test2),
            Ok(RespData::SimpleStr(
                "Hello World this Has Upper And LowerCase LEtTTers".into()
            ))
        );

        let test3 = String::from("+12345\r\n");

        assert_eq!(
            RespData::from_string(test3),
            Ok(RespData::SimpleStr("12345".into()))
        );
    }

    #[test]
    fn it_parses_error_strings() {
        let test1 = String::from("-Error\r\n");

        assert_eq!(
            RespData::from_string(test1),
            Ok(RespData::Error("Error".into()))
        );
    }

    #[test]
    fn it_parses_numbers() {
        let test1 = String::from(":100\r\n");
        let test2 = String::from(":-100\r\n");
        let test3 = String::from(":invalidnumber\r\n");
        assert_eq!(RespData::from_string(test1), Ok(RespData::Number(100)));
        assert_eq!(RespData::from_string(test2), Ok(RespData::Number(-100)));
        assert_eq!(
            RespData::from_string(test3),
            Err("Can't parse integer".into())
        );
    }

    #[test]
    fn it_parses_bulk_strings() {
        let test1 = String::from("$5\r\nHELLO\r\n");
        assert_eq!(
            RespData::from_string(test1),
            Ok(RespData::BulkStr("HELLO".into()))
        );

        // Police test
        let test2 = String::from("$15\r\nHELLOHELLOHELLO\r\n");
        assert_eq!(
            RespData::from_string(test2),
            Ok(RespData::BulkStr("HELLOHELLOHELLO".into()))
        );
    }
}
