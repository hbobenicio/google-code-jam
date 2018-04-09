use std::io;
use std::io::prelude::BufRead;

#[derive(Debug)]
pub struct Scanner {
    stdin: io::Stdin,
    capacity: usize,
    str_buffer: String
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            stdin: io::stdin(),
            capacity: 0,
            str_buffer: String::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Scanner {
        Scanner {
            stdin: io::stdin(),
            capacity,
            str_buffer: String::with_capacity(capacity)
        }
    }

    pub fn read_line(&mut self) -> String {
        self.str_buffer.clear();

        self.stdin
            .read_line(&mut self.str_buffer)
            .expect("Error while reading line from stdin");

        String::from(self.str_buffer.trim())
    }

    pub fn read_line2(&mut self) -> Result<String, io::Error> {
        self.str_buffer.clear();

        self.stdin.read_line(&mut self.str_buffer)?;

        Ok(String::from(self.str_buffer.trim()))
    }

    /// Reads all lines from stdin.
    /// This ignores empty lines.
    pub fn all_lines<F>(&self, f: F)
    where
        F: Fn((usize, String))
    {
        for x in self.stdin.lock().lines()
            .map(|l| l.expect("gcj::utils - Error while reading line from stdin"))
            .filter(|l| l.trim() != "")
            .enumerate()
            .map(|(i, l)| (i+1, l))
        {
            f(x);
        };
    }
}
