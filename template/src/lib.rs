//! Copy this crate to `aocN` for a new day, then:
//!
//!   - rename the package in `Cargo.toml` and the struct below,
//!   - set `DAY`,
//!   - add `aocN = { path = "../aocN" }` to `aoc/Cargo.toml`,
//!   - add `Entry::of::<aocN::DayN>()` to `DAYS` in `aoc/src/main.rs`.
//!
//! Parsing belongs in `from_str` and should reject anything the solve would
//! rather not think about. Replace `lines` with whatever shape the day wants.

use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day {
    lines: Vec<String>,
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        Ok(Day {
            lines: input
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(str::to_string)
                .collect(),
        })
    }
}

impl Solution for Day {
    const DAY: u8 = 0;

    fn solve(self) -> Result<Answer, Error> {
        Ok(Answer::new(self.lines.len(), "unsolved"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    #[test]
    fn puzzle_sample() {
        let input = sample(&["one", "two"]);
        let result = input.parse::<Day>().unwrap().solve().unwrap();

        assert_eq!(result.part_1, "2");
    }
}
