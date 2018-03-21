//! Tidy
//!
//! TODO - Document this module

use std::char;

pub enum TidyEnum {
    Tidy,
    NotTidy(u8)
}

// pub enum TidyEnum2 {
//     Tidy,
//     NotTidy {i: u8}
// }

use self::TidyEnum::{Tidy, NotTidy};

pub fn solve(line: &str) -> String {
    // Casting the line to a Vec of algorisms
    let algorisms: Vec<u8> = str_to_vec(line);

    // Check if it's a tidy number or not.
    // If it's not, gives you also the index of the first "slope-down" index
    match tidy_check(&algorisms) {
        Tidy => String::from(line),
        NotTidy(i) => cast_previous_tidy(&algorisms, i)
    }
}

fn char_to_digit(c: char) -> u8 {
    c.to_digit(10)
        .expect("Error while converting char") as u8
}

fn digit_to_char(d: &u8) -> char {
    char::from_digit(*d as u32, 10)
        .expect("Error while converting from digit")
}

pub fn str_to_vec(s: &str) -> Vec<u8> {
    s.chars().map(char_to_digit).collect()
}

pub fn vec_to_str(v: &Vec<u8>) -> String {
    v.iter()
        .map(digit_to_char)
        .fold(String::new(), |mut s, c| {
            s.push(c);
            s
        })
}

pub fn tidy_check(s: &Vec<u8>) -> TidyEnum {
    let mut i = 0;
    while i < s.len() - 1 {
        let j = i + 1;
        let x = s.get(i..i+1).unwrap();
        let y = s.get(j..j+1).unwrap();

        if x > y {
            break;
        }

        i += 1;
    }

    match i == s.len() - 1 {
        true => TidyEnum::Tidy,
        false => {
            while i > 0 {
                let j = i - 1;
                let x = s.get(i..i+1).unwrap();
                let y = s.get(j..j+1).unwrap();
                if x != y {
                    return TidyEnum::NotTidy(i as u8);
                }
                i -= 1;
            }

            TidyEnum::NotTidy(0)
        }
    }
}

fn remove_left_zeros(v: &Vec<u8>) -> Vec<u8> {
    let mut resp: Vec<u8> = Vec::with_capacity(v.len());

    let mut v_iter = v.iter().peekable();
    while let Some(&&0) = v_iter.peek() {
        v_iter.next();
    }

    while let Some(&x) = v_iter.next() {
        resp.push(x);
    }

    resp

    // let mut iter = v.iter().skip_while(|x| **x == 0);
    // iter.collect()
}

pub fn cast_previous_tidy(s: &Vec<u8>, slope_index: u8) -> String {
    let mut v: Vec<u8> = Vec::with_capacity(s.len());

    for (i, x) in s.iter().enumerate() {
        if i == slope_index as usize {
            v.push(x - 1);
        } else if i > slope_index as usize {
            v.push(9);
        } else {
            v.push(*x);
        }
    }

    v = remove_left_zeros(&v);

    vec_to_str(&v)
}
