use super::{ErrorKind, ErrorKind::*};
use std::convert::TryFrom;

pub struct Error {
    column: u16, //No line should have more than 60 thousand columns, you'd have to try really hard to accomplish this.
    kind: ErrorKind,
}

pub struct ParsedLine {} //Only partially parsed, of course, some errors can only be detected when knowing multiple lines.

pub fn parse_line(line: &str) -> Result<ParsedLine, Error> {
    match indentation(line) {
        Ok(indented_line) => unimplemented!(),
        Err(kind) => Err(Error { column: 0, kind }),
    }
}

struct IndentedLine<'a> {
    pub tabs: u8, //If you have more than 255 tabs at the start of a line, there is something wrong with your code.
    pub text: &'a str,
}

fn indentation(line: &str) -> Result<IndentedLine, ErrorKind> {
    for (i, &byte) in line.as_bytes().iter().enumerate() {
        if byte != b'\t' {
            return Ok(IndentedLine {
                tabs: if let Ok(count) = u8::try_from(i) {
                    count
                } else {
                    return Err(TooManyTabs);
                },
                text: &line[i as usize..],
            });
        }
    }
    Ok(IndentedLine {
        tabs: line.len() as u8,
        text: &line[line.len() - 1..], //empty string, defined like this for consistency and to avoid lifetime problems
    })
}
