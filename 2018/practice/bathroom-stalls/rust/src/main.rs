// BufRead trait is needed for stdin.lock().lines()
use std::io::{self, BufRead};

mod stall;

#[derive(Debug)]
struct Input {
    i: u64,
    n: u64,
    k: u64
}

fn test_case(input: Input) {
    let (y, z) = solve(&input);
    println!("Case #{}: {} {}", input.i, y, z);
}

fn solve(input: &Input) -> (u64, u64) {
    let mut _stalls = stall::new_stalls(input.n);

    let (y, z) = (0, 0);
    (y, z)
}

fn main() {
    // println!("std::u64::MAX = {}", std::u64::MAX);
    // println!("std::usize::MAX = {}", std::usize::MAX);

    let stdin = io::stdin();
    let mut line_buffer = String::with_capacity(100);

    stdin.read_line(&mut line_buffer).expect("Error while reading from stdin");
    let t = line_buffer.trim().parse::<u64>().expect("Error while parsing u64 from line");

    for i in 1..t+1 {
        // line_buffer.clear();

        stdin.lock().read_line(&mut line_buffer).unwrap();

        let mut split_iter = line_buffer.trim().split_whitespace();
        let n = split_iter.next()
            .map(|x| x.parse::<u64>().expect("Error while parsing u64 from input N"))
            .expect("No N value found on stdin line");
        let k = split_iter.next()
            .map(|x| x.parse::<u64>().expect("Error while parsing u64 from input K"))
            .expect("No K value found on stdin line");

        test_case(Input{i, n, k});
    }
}
