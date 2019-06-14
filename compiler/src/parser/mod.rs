mod line;

pub use line::{parse_line, Error, ParsedLine};

use crate::error;
use error::PrintableError;

pub enum ErrorKind {
    TooManyTabs,
}

impl PrintableError for ErrorKind {
    fn id(&self) -> error::ID {
        use ErrorKind::*;
        match self {
            TooManyTabs => 0,
        }
    }
    fn msg(&self) -> String {
        use ErrorKind::*;
        match self {
            TooManyTabs => String::from(
                "\
There are more than 255 tabs in front of this line.
It's likely that you should split up your logic into more functions.",
            ),
        }
    }
}
