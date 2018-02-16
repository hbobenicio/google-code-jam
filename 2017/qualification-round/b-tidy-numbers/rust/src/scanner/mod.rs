use std::io;

#[derive(Debug)]
pub struct Scanner {
    capacity: usize,
    buffer: String,
}

impl Scanner {
    pub fn new(capacity: usize) -> Scanner {
        Scanner {
            capacity,
            buffer: String::with_capacity(capacity)
        }
    }

    pub fn from_stdio_get_i32(&mut self) -> i32 {
        io::stdin()
            .read_line(&mut self.buffer)
            .unwrap();

        self.buffer
            .trim()
            .parse::<i32>()
            .unwrap()
    }
}
