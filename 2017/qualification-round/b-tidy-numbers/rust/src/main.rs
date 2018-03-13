#[allow(dead_code)]
#[allow(unused_imports)]

use std::io;

struct Num {
    integer: i32,
    string: String,
}

impl Num {

    fn from_integer(x: i32) -> Num {
        Num {
            integer: x,
            string: x.to_string()
        }
    }

    fn from_string(str: &str) -> Num {
        Num {
            integer: str.parse::<i32>().unwrap(),
            string: String::from(str)
        }
    }
}

fn is_tidy(num_str: &String) -> bool {
    unimplemented!();
}

fn find_max_tidy(num: &Num) {
    unimplemented!();
}

fn main() {
    // The line buffer
    let mut line = String::with_capacity(100);

    // Reads T (Number of Test Cases)
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        // Reads the ith number. Casting it to Num object
        // (which acts like unions...)
        io::stdin().read_line(&mut line).unwrap();
        let num = Num::from_string(line.trim());

        find_max_tidy(&num);
    }
}
