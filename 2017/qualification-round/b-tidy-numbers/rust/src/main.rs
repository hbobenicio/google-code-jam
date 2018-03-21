#[allow(dead_code)]
#[allow(unused_imports)]

mod tidy;
mod gcj;

use std::io;

/// # [Tidy Numbers](https://code.google.com/codejam/contest/3264486/dashboard#s=p1)
///
/// ## Contest
///
/// Qualification Round 2017
///
/// ## Algorithm Description
///
/// Algorithm:
/// 1. Traverse the input string to find the "slope-down" index
/// 2.   if this index == string.len(), then it's a tidy number. just print it
/// 3. While char(i-1) == char(i), i--. Let's call it "updated-slopw-down" index.
///    (shift left the index while there are equal numbers on the left)
/// 4. With this new index, get the higher tidy number lower then the input value by:
///   4.1. Decrease the algorism at this index by 1
///   4.2. Set all the algorisms on the right of this index to '9'
/// 5. Cast it to number and print (to get rid of left zeros)
fn main() {

    // The line buffer
    let mut line = String::with_capacity(100);

    // Reads T (Number of Test Cases)
    io::stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<u8>().unwrap();

    // let stdin = io::stdin();
    // for (i, s) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
    //     println!("Case #{}: {}", i + 1, tidy::solve(&s));
    // }

    // for test_case in 1..t+1 {
    //     // Clears the data of the next read_line, but preserves its capacity
    //     line.clear();
    //
    //     // Reads the ith number
    //     io::stdin().read_line(&mut line).unwrap();
    //
    //     println!("Case #{}: {}", test_case, tidy::solve(line.trim()));
    // }

    let scanner = gcj::io::Scanner::with_capacity(100);

    scanner.all_lines(|(i, line)|
        println!("Case #{}: {}", i, tidy::solve(line.trim()))
    )
}
