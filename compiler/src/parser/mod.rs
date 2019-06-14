mod line;

pub use line::{parse_line, Error, ParsedLine};

pub enum ErrorKind {
    TooManyTabs,
}
