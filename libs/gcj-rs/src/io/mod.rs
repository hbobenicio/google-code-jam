pub mod scanner;

use std::io::{self, BufRead};

// Exporting gcj_rs::io::scanner::Scanner to gcj_rs::io::Scanner
pub use self::scanner::Scanner;

pub fn readline(input: &io::Stdin) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let mut handle = input.lock();

    handle.read_line(&mut buffer)?;

    let response = String::from(buffer.trim());
    Ok(response)
}
