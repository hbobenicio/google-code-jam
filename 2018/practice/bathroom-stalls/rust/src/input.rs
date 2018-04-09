#[derive(Debug)]
pub struct Input {
    pub i: u64,
    pub n: u64,
    pub k: u64
}

impl Input {
    pub fn new(i: u64, n: u64, k: u64) -> Input {
        Input { i, n, k }
    }
}
