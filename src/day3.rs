use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const PATH: &str = "data/day-3.txt";

pub fn run() {
    let file = File::open(PATH).expect("cant open file");
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line in reader.lines() {
        let value = calc_line(&line.unwrap());
        total += value;
    }

    println!("{}", total);
}

fn calc_line(line: &str) -> i32 {
    let (left, right) = line.split_at(line.len() / 2);
    let set: HashSet<char> = HashSet::from_iter(left.chars());
    for ch in right.chars() {
        if set.contains(&ch) {
            return get_char_value(&ch);
        }
    }
    return 0;
}

fn get_char_value(ch: &char) -> i32 {
    let acode = 'a' as u8 as i32;
    let zcode = 'z' as u8 as i32;
    let a_cap_code = 'A' as u8 as i32;
    // let z_cap_code = 'Z' as u8 as i32;
    let code = *ch as u8 as i32;
    if code >= acode && code <= zcode {
        return code - acode + 1;
    }

    println!("char {}, code {}, cap {}", ch, code, a_cap_code);
    return code - a_cap_code + 27;
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn test_get_char_value() {
        // assert_eq!(get_char_value(&'a'), 1);
        // assert_eq!(get_char_value(&'b'), 2);
        // assert_eq!(get_char_value(&'z'), 26);

        assert_eq!(get_char_value(&'A'), 27);
        assert_eq!(get_char_value(&'B'), 28);
        assert_eq!(get_char_value(&'Z'), 52);
    }
}
