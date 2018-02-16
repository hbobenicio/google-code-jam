#[allow(dead_code)]
#[allow(unused_imports)]

mod gcj;
mod scanner;

use std::io;

fn main() {
    // let capacity = 2500;
    // let mut data = String::with_capacity(capacity);

    // io::stdin()
    //     .read_to_string(&mut data).unwrap();

    // let mut lines = data.lines();
    // let _t = lines
    //     .next().unwrap()
    //     .parse::<i32>().unwrap();

    // for (i, line) in lines.enumerate() {
    //     let n = line.parse::<u64>().unwrap();
    //     println!("Case #{}: {}", i, n);
    // };

    // let mut scanner = scanner::Scanner::new(2500);
    // let x: i32 = scanner.from_stdio_get_i32();
    // println!("{:?}", x);

    let v: Vec<&str> = vec![];
    let primeiro = v.first();
    scanner::foo().unwrap();
    println!("Primeiro elemento: {:?}", primeiro);
}
