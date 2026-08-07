use std::cmp::Ordering;
use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day5 {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl FromStr for Day5 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut ranges = Vec::new();
        let mut ids = Vec::new();

        for line in input.lines().filter(|line| !line.trim().is_empty()) {
            let line = line.trim();

            match line.split_once('-') {
                Some((start, end)) => ranges.push((number(start)?, number(end)?)),
                None => ids.push(number(line)?),
            }
        }

        Ok(Day5 {
            ranges: merge(ranges),
            ids,
        })
    }
}

fn number(text: &str) -> Result<usize, Error> {
    text.parse()
        .map_err(|_| Error::parse(format!("not a number: {text}")))
}

impl Solution for Day5 {
    const DAY: u8 = 5;

    fn solve(self) -> Result<Answer, Error> {
        let part_1 = self
            .ids
            .iter()
            .filter(|id| covers(&self.ranges, **id))
            .count();

        let part_2: usize = self.ranges.iter().map(|(start, end)| end - start + 1).sum();

        Ok(Answer::new(part_1, part_2))
    }
}

/// Sorts and joins overlapping ranges, leaving them ready to binary search.
pub fn merge(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|range| range.0);

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn covers(ranges: &[(usize, usize)], id: usize) -> bool {
    ranges
        .binary_search_by(|(start, end)| {
            if id < *start {
                Ordering::Greater
            } else if id > *end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    #[test]
    fn merges_overlaps() {
        assert_eq!(
            merge(vec![(10, 14), (16, 20), (12, 18), (3, 5)]),
            vec![(3, 5), (10, 20)]
        );
    }

    #[test]
    fn finds_ids_in_merged_ranges() {
        let ranges = merge(vec![(10, 14), (16, 20)]);
        assert!(covers(&ranges, 10));
        assert!(covers(&ranges, 20));
        assert!(!covers(&ranges, 9));
        assert!(!covers(&ranges, 15));
        assert!(!covers(&ranges, 21));
    }

    #[test]
    fn puzzle_sample() {
        let input = sample(&["3-5", "10-14", "16-20", "12-18", "10-20", "1", "2", "3"]);
        let result = input.parse::<Day5>().unwrap().solve().unwrap();

        assert_eq!(result.part_1, "1");
        assert_eq!(result.part_2, "14");
    }
}
