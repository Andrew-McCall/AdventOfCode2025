use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day2 {
    ranges: Vec<(usize, usize)>,
}

impl FromStr for Day2 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let ranges = input
            .split(',')
            .map(str::trim)
            .filter(|range| !range.is_empty())
            .map(parse_range)
            .collect::<Result<_, _>>()?;

        Ok(Day2 { ranges })
    }
}

fn parse_range(range: &str) -> Result<(usize, usize), Error> {
    let (start, end) = range
        .split_once('-')
        .ok_or_else(|| Error::parse(format!("not a range: {range}")))?;

    let start = start
        .parse()
        .map_err(|_| Error::parse(format!("not a number: {start}")))?;
    let end = end
        .parse()
        .map_err(|_| Error::parse(format!("not a number: {end}")))?;

    if start > end {
        return Err(Error::parse(format!("backwards range: {range}")));
    }

    Ok((start, end))
}

impl Solution for Day2 {
    const DAY: u8 = 2;

    fn solve(self) -> Result<Answer, Error> {
        let mut part_1 = 0;
        let mut part_2 = 0;

        for (start, end) in self.ranges {
            for number in start..=end {
                let digits = number.to_string();
                let digits = digits.as_bytes();

                if is_doubled(digits) {
                    part_1 += number;
                }
                if is_repeated(digits) {
                    part_2 += number;
                }
            }
        }

        Ok(Answer::new(part_1, part_2))
    }
}

/// The digits are one half written twice, e.g. `123123`.
fn is_doubled(digits: &[u8]) -> bool {
    let half = digits.len() / 2;
    digits.len().is_multiple_of(2) && digits[..half] == digits[half..]
}

/// The digits are some shorter run repeated to fill them, e.g. `121212`.
fn is_repeated(digits: &[u8]) -> bool {
    let length = digits.len();
    if length < 2 {
        return false;
    }

    (1..=length / 2)
        .filter(|size| length.is_multiple_of(*size))
        .any(|size| digits.chunks(size).all(|chunk| chunk == &digits[..size]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(input: &str) -> Answer {
        input.parse::<Day2>().unwrap().solve().unwrap()
    }

    #[test]
    fn spots_doubles() {
        assert!(is_doubled(b"123123"));
        assert!(is_doubled(b"11"));
        assert!(!is_doubled(b"1231234"));
        assert!(!is_doubled(b"123124"));
        assert!(!is_doubled(b"7"));
    }

    #[test]
    fn spots_repeats() {
        assert!(is_repeated(b"121212"));
        assert!(is_repeated(b"1111"));
        assert!(is_repeated(b"123123"));
        assert!(!is_repeated(b"123124"));
        assert!(!is_repeated(b"7"));
    }

    #[test]
    fn sums_a_range() {
        // 11 and 22 are doubles; 111 is a repeat but not a double.
        let result = solve("10-30,111-111");
        assert_eq!(result.part_1, "33");
        assert_eq!(result.part_2, "144");
    }

    #[test]
    fn rejects_junk() {
        assert!("10-".parse::<Day2>().is_err());
        assert!("10".parse::<Day2>().is_err());
        assert!("30-10".parse::<Day2>().is_err());
    }
}
