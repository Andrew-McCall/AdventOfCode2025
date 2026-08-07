use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day7 {
    _lines: Vec<String>,
}

impl FromStr for Day7 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        Ok(Day7 {
            _lines: input.lines().map(str::to_string).collect(),
        })
    }
}

impl Solution for Day7 {
    const DAY: u8 = 7;

    fn solve(self) -> Result<Answer, Error> {
        Ok(Answer::new("unsolved", "unsolved"))
    }
}
