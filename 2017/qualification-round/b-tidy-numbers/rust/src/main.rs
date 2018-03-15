#[allow(dead_code)]
#[allow(unused_imports)]

use std::io;

#[derive(Debug, Clone)]
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

    fn from_string(s: &str) -> Num {
        Num {
            integer: s.parse::<i32>().unwrap(),
            string: String::from(s)
        }
    }

    fn is_tidy(&self) -> bool {
        for i in 0..self.string.len()-1 {
            let j = i + 1;
            let x = self.string.get(i..i+1).unwrap();
            let y = self.string.get(j..j+1).unwrap();
            let xi = x.parse::<i32>().unwrap();
            let yi = y.parse::<i32>().unwrap();

            if xi > yi {
                return false
            }
        }
        true
    }
}

fn find_max_tidy(num: &Num) {
    // if num.string.len() <= 1 || is_tidy(num) {
    //     println!("{}", num.integer);
    // } else {
    //     find_max_tidy(&Num::from_integer(num.integer - 1));
    // }

    let mut x: Num = num.clone();
    while !x.is_tidy() {
        x = Num::from_integer(x.integer - 1);
    }

    println!("{}", x.integer);
}

fn main() {
    // The line buffer
    let mut line = String::with_capacity(100);

    // Reads T (Number of Test Cases)
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        line.clear();

        // Reads the ith number. Casting it to Num object
        // (which acts like unions...)
        io::stdin().read_line(&mut line).unwrap();
        let num = Num::from_string(line.trim());

        find_max_tidy(&num);
    }
}
