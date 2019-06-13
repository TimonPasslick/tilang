use crate::ast::AST;
use std::{convert::TryFrom, io, io::BufRead};

pub type LineNumber = u16; //Source files really shouldn't have more than 60 thousand lines.

pub enum Error {
    TooManyTabs {
        line_number: LineNumber,
    },
    LineUnreadable {
        number: LineNumber,
        io_error: io::Error,
    },
}

pub fn parse(input_stream: &mut impl BufRead) -> Result<AST, Error> {
    for (line, line_number) in input_stream.lines().zip(0 as LineNumber..) {
        match line {
            Ok(line) => match indentation(&line) {
                Ok(indented_line) => unimplemented!(),
                Err(IndentationError::TooManyTabs) => {
                    return Err(Error::TooManyTabs { line_number })
                }
            },
            Err(error) => {
                return Err(Error::LineUnreadable {
                    number: line_number,
                    io_error: error,
                })
            }
        }
    }
    unimplemented!()
}

struct IndentedLine<'a> {
    pub tabs: u8, //If you have more than 255 tabs at the start of a line, there is something wrong with your code.
    pub text: &'a str,
}
enum IndentationError {
    TooManyTabs,
}
fn indentation(line: &str) -> Result<IndentedLine, IndentationError> {
    for (i, &byte) in line.as_bytes().iter().enumerate() {
        if byte != b'\t' {
            return Ok(IndentedLine {
                tabs: if let Ok(count) = u8::try_from(i) {
                    count
                } else {
                    return Err(IndentationError::TooManyTabs);
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
