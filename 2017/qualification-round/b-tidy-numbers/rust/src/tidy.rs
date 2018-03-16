pub enum TidyEnum {
    Tidy,
    NotTidy(u8)
}

pub enum TidyEnum2 {
    Tidy,
    NotTidy {i: u8}
}

fn char_to_digit(c: char) -> u8 {
    c.to_digit(10)
        .expect("Error while converting char") as u8
}

pub fn str_to_vec(s: &str) -> Vec<u8> {
    s.chars().map(char_to_digit).collect()
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

pub fn cast_previous_tidy(s: &Vec<u8>, slope_index: u8) -> Vec<u8> {
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

    v
}
