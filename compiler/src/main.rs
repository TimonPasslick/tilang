mod ast;
mod parser;

use std::{fs::File, io::BufReader};

fn error_message(msg: &str) -> ! {
    eprintln!("[ERR] {}", msg);
    std::process::exit(1);
}

fn main() {
    let file = File::open("../examples/vec2.ti").unwrap_or_else(|e| {
        error_message(&format!(
            "The specified file could not be opened, system error: {}",
            e
        ));
    });
    let ast = parser::parse(&mut BufReader::new(file)).unwrap_or_else(|e| {
        use parser::Error::*;
        match e {
            TooManyTabs{line_number} => error_message(&format!("line {:3}: More than 255 tabs at the start of a line are forbidden, you might want to divide your code into smaller parts.", line_number)),
            LineUnreadable{number, io_error} => error_message(&format!("line {:3}: This line can't be read, exact error: {}", number, io_error))
        }
    });
}
