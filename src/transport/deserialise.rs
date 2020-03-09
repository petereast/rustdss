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
[ "LLEN", "mylist" ]

*/

#[derive(Debug, PartialEq)]
enum RespData {
    Error(String),       // Errors are just text
    Number(i64),         // Numbers
    SimpleStr(String),   // Simple strings are not prefixed with length
    BulkStr(String),     // BulkStr is prefixed with it's length
    List(Vec<RespData>), // Lists don't have to be made up of the same type
}

fn parse_resp_value(value: String) -> Result<RespData, String> {
    // TODO: Strip off the whitespace at the end -- or not?
    let mut chars = value.chars();

    fn parse_bulk_string(val: &mut std::str::Chars) -> RespData {
        // Bulk strings are in the following format:
        // "$10\r\nabcdefghij\r\n"
        // This function will recieve this:
        // "10\r\nabcdefghij\r\n"
        // 10 is the length of the string
        // We read the number, and then `take` the next n characters
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
        // Do something with the value we extracted:
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

    Ok(match chars.next().unwrap_or('?') {
        '-' => RespData::Error(String::from(chars.as_str().trim())),
        ':' => RespData::Number(
            String::from(chars.as_str())
                .trim()
                .parse()
                .map_err(|_| "Can't parse integer")?,
        ),
        '+' => RespData::SimpleStr(String::from(chars.as_str().trim())),
        '$' => parse_bulk_string(&mut chars),
        '*' => unimplemented!(), // Parse list of RESP values
        _ => RespData::Error("Unknown character".into()),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_simple_strings() {
        let test1 = String::from("+HELLO\r\n");

        assert_eq!(
            parse_resp_value(test1),
            Ok(RespData::SimpleStr("HELLO".into()))
        );

        let test2 = String::from("+Hello World this Has Upper And LowerCase LEtTTers\r\n");

        assert_eq!(
            parse_resp_value(test2),
            Ok(RespData::SimpleStr(
                "Hello World this Has Upper And LowerCase LEtTTers".into()
            ))
        );

        let test3 = String::from("+12345\r\n");

        assert_eq!(
            parse_resp_value(test3),
            Ok(RespData::SimpleStr("12345".into()))
        );
    }

    #[test]
    fn it_parses_error_strings() {
        let test1 = String::from("-Error\r\n");

        assert_eq!(parse_resp_value(test1), Ok(RespData::Error("Error".into())));
    }

    #[test]
    fn it_parses_numbers() {
        let test1 = String::from(":100\r\n");
        let test2 = String::from(":-100\r\n");
        let test3 = String::from(":invalidnumber\r\n");
        assert_eq!(parse_resp_value(test1), Ok(RespData::Number(100)));
        assert_eq!(parse_resp_value(test2), Ok(RespData::Number(-100)));
        assert_eq!(parse_resp_value(test3), Err("Can't parse integer".into()));
    }

    #[test]
    fn it_parses_bulk_strings() {
        let test1 = String::from("$5\r\nHELLO\r\n");
        assert_eq!(
            parse_resp_value(test1),
            Ok(RespData::BulkStr("HELLO".into()))
        );

        // Police test
        let test2 = String::from("$15\r\nHELLOHELLOHELLO\r\n");
        assert_eq!(
            parse_resp_value(test2),
            Ok(RespData::BulkStr("HELLOHELLOHELLO".into()))
        );
    }
}
