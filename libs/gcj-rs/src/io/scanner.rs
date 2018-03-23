use std::io;
use std::io::prelude::BufRead;

#[derive(Debug)]
pub struct Scanner {
    stdin: io::Stdin,
    capacity: usize
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            stdin: io::stdin(),
            capacity: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> Scanner {
        Scanner {
            stdin: io::stdin(),
            capacity
        }
    }

    pub fn read_line(&self) -> String {
        let mut line = String::with_capacity(self.capacity);

        self.stdin
            .read_line(&mut line)
            .expect("Error while reading line from stdin");

        String::from(line.trim())
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
