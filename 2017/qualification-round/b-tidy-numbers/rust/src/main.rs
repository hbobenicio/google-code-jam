#[allow(dead_code)]
#[allow(unused_imports)]

mod tidy;

use std::io;
use tidy::TidyEnum::*;

fn main() {
    // Algorithm:
    // 1. Traverse the input string to find the "slope-down" index
    // 2.   if this index == string.len(), then it's a tidy number. just print it
    // 3. While char(i-1) == char(i), i--. Let's call it "updated-slopw-down" index.
    //    (shift left the index while there are equal numbers on the left)
    // 4. With this new index, get the higher tidy number lower then the input value by:
    //   4.1. Decrease the algorism at this index by 1
    //   4.2. Set all the algorisms on the right of this index to '9'
    // 5. Cast it to number and print (to get rid of left zeros)

    // The line buffer
    let mut line = String::with_capacity(100);

    // Reads T (Number of Test Cases)
    io::stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<u8>().unwrap();

    for _ in 0..t {
        // Clears the data of the next read_line, but preserves its capacity
        line.clear();

        // Reads the ith number
        io::stdin().read_line(&mut line).unwrap();

        // Casting the line to a Vec of algorisms
        let algorisms = tidy::str_to_vec(line.trim());

        // Check if it's a tidy number or not.
        // If it's not, gives you also the index of the first "slope-down" index
        match tidy::tidy_check(&algorisms) {
            Tidy => println!("{}", line.trim()),
            NotTidy(i) => {
                println!("{:?}", tidy::cast_previous_tidy(&algorisms, i));
            }
        };
    }
}
