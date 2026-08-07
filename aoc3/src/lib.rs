use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day3 {
    banks: Vec<Vec<u8>>,
}

impl FromStr for Day3 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut banks = Vec::new();

        for line in input.lines().filter(|line| !line.trim().is_empty()) {
            let line = line.trim();
            if !line.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(Error::parse(format!("not all digits: {line}")));
            }
            banks.push(line.as_bytes().to_vec());
        }

        Ok(Day3 { banks })
    }
}

impl Solution for Day3 {
    const DAY: u8 = 3;

    fn solve(self) -> Result<Answer, Error> {
        let mut part_1: u64 = 0;
        let mut part_2: u64 = 0;

        for bank in &self.banks {
            part_1 += joltage(bank, 2)?;
            part_2 += joltage(bank, 12)?;
        }

        Ok(Answer::new(part_1, part_2))
    }
}

/// The largest number of `size` digits that can be read off `digits` in order.
/// Take the biggest digit that still leaves enough behind, then repeat.
fn joltage(digits: &[u8], size: usize) -> Result<u64, Error> {
    if digits.len() < size {
        return Err(Error::solve(format!(
            "need {size} digits, bank only has {}",
            digits.len()
        )));
    }

    let mut value = 0;
    let mut start = 0;

    for taken in 0..size {
        let window = &digits[start..digits.len() - size + taken + 1];
        let (offset, digit) = first_largest(window);

        value = value * 10 + u64::from(digit - b'0');
        start += offset + 1;
    }

    Ok(value)
}

/// Position and value of the largest byte, preferring the earliest on a tie.
fn first_largest(digits: &[u8]) -> (usize, u8) {
    let mut largest = (0, digits[0]);

    for (index, &digit) in digits.iter().enumerate() {
        if digit > largest.1 {
            largest = (index, digit);
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    fn solve(lines: &[&str]) -> Answer {
        sample(lines).parse::<Day3>().unwrap().solve().unwrap()
    }

    #[test]
    fn takes_the_earliest_largest() {
        assert_eq!(first_largest(b"1919"), (1, b'9'));
    }

    #[test]
    fn one_bank() {
        let result = solve(&["897654321111111119"]);
        assert_eq!(result.part_1, "99");
        assert_eq!(result.part_2, "976543211119");
    }

    #[test]
    fn puzzle_sample() {
        let result = solve(&[
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]);
        assert_eq!(result.part_1, "357");
        assert_eq!(result.part_2, "3121910778619");
    }

    #[test]
    fn rejects_junk() {
        assert!("12x4".parse::<Day3>().is_err());
    }

    #[test]
    fn reports_short_banks() {
        assert!("123".parse::<Day3>().unwrap().solve().is_err());
    }
}
