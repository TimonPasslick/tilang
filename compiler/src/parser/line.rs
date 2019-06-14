use super::{ErrorKind, ErrorKind::TooManyTabs};
use std::convert::TryFrom;

pub struct Error {
    column: u16, //No line should have more than 60 thousand columns, you'd have to try really hard to accomplish this.
    kind: ErrorKind,
}

pub struct ParsedLine {} //Only partially parsed, of course, some errors can only be detected when knowing multiple lines.

pub fn parse_line(line: &str) -> Result<ParsedLine, Error> {
    let indented = indentation(line)?;
    unimplemented!()
}

enum IndentedLine<'a> {
    WithCode {
        tabs: u8, //If you have more than 255 tabs at the start of a line, there is something wrong with your code.
        text: &'a str,
    },
    Empty,
}

fn indentation(line: &str) -> Result<IndentedLine, Error> {
    for (i, &byte) in line.as_bytes().iter().enumerate() {
        if byte != b'\t' {
            return Ok(IndentedLine::WithCode {
                tabs: if let Ok(count) = u8::try_from(i) {
                    count
                } else {
                    return Err(Error {
                        column: 0,
                        kind: TooManyTabs,
                    });
                },
                text: &line[i as usize..],
            });
        }
    }
    Ok(IndentedLine::Empty)
}
