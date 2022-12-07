use std::fs::File;
use std::io::{prelude::*, BufReader};

const PATH: &str = "data/day-2.txt";

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Tie,
}

impl Choice {
    fn from(s: &str) -> Choice {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Wrong input"),
        }
    }

    fn against(&self, opponent: &Self) -> Outcome {
        match (self, opponent) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Scissors, Choice::Paper)
            | (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Scissors, Choice::Rock)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Rock, Choice::Paper) => Outcome::Lose,
            _ => Outcome::Tie,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

pub fn run() {
    let file = File::open(PATH).expect("cant open file");
    let reader = BufReader::new(file);

    let mut acc: i32 = 0;

    for line in reader.lines() {
        let text = line.expect("reading line");
        let mut iter = text.split_whitespace();
        let left = Choice::from(iter.next().unwrap());
        let right = Choice::from(iter.next().unwrap());
        acc += score(left, right);
    }

    print!("Scores: {}\n", acc);
}

fn score(opponent: Choice, mine: Choice) -> i32 {
    let outcome = mine.against(&opponent);
    let mut total = match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Tie => 3,
    };
    total += mine.value();

    return total;
}

#[cfg(test)]
mod tests {
    use crate::day2::Choice;

    #[test]
    fn choice_from_str() {
        let result = Choice::from("A");
        matches!(result, Choice::Rock);
    }
}
