use std::fs::File;
use std::io::{prelude::*, BufReader};

const PATH: &str = "data/day-1.txt";

fn main() {
    let file = File::open(PATH).expect("cant open file");
    let reader = BufReader::new(file);

    let mut max: i32 = 0;
    let mut acc: i32 = 0;

    for line in reader.lines() {
        let text = line.expect("reading line");
        if text == "" {
            if acc > max {
                max = acc;
            }

            acc = 0;
        }

        let num: i32 = text.parse().unwrap_or_default();
        acc = acc + num;
    }

    print!("{}", max);
}
