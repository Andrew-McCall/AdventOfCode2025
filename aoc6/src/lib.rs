use std::ops::Range;
use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day6 {
    problems: Vec<Problem>,
}

struct Problem {
    rows: Vec<Vec<u8>>,
    operator: Operator,
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Day6 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut rows: Vec<&[u8]> = input
            .lines()
            .filter(|row| !row.trim().is_empty())
            .map(str::as_bytes)
            .collect();

        let operators = rows
            .pop()
            .ok_or_else(|| Error::parse("worksheet has no rows"))?;

        if rows.is_empty() {
            return Err(Error::parse("worksheet has operators but no numbers"));
        }

        let width = rows
            .iter()
            .chain([&operators])
            .map(|row| row.len())
            .max()
            .unwrap_or(0);

        let problems = blocks(&rows, operators, width)
            .into_iter()
            .map(|block| Problem::read(&rows, operators, block))
            .collect::<Result<_, _>>()?;

        Ok(Day6 { problems })
    }
}

fn at(row: &[u8], column: usize) -> u8 {
    row.get(column).copied().unwrap_or(b' ')
}

fn blocks(rows: &[&[u8]], operators: &[u8], width: usize) -> Vec<Range<usize>> {
    let blank = |column| {
        rows.iter()
            .chain([&operators])
            .all(|row| at(row, column) == b' ')
    };

    let mut blocks = Vec::new();
    let mut start = None;

    for column in 0..width {
        match (blank(column), start) {
            (false, None) => start = Some(column),
            (true, Some(from)) => {
                blocks.push(from..column);
                start = None;
            }
            _ => {}
        }
    }

    if let Some(from) = start {
        blocks.push(from..width);
    }

    blocks
}

impl Problem {
    fn read(rows: &[&[u8]], operators: &[u8], block: Range<usize>) -> Result<Self, Error> {
        let symbols: Vec<u8> = block
            .clone()
            .map(|column| at(operators, column))
            .filter(|symbol| *symbol != b' ')
            .collect();

        let operator = match symbols.as_slice() {
            [b'+'] => Operator::Add,
            [b'*'] => Operator::Multiply,
            [other] => {
                return Err(Error::parse(format!(
                    "unknown operator: {}",
                    *other as char
                )));
            }
            [] => {
                return Err(Error::parse(format!(
                    "problem at column {} has no operator",
                    block.start
                )));
            }
            _ => {
                return Err(Error::parse(format!(
                    "problem at column {} has more than one operator",
                    block.start
                )));
            }
        };

        let rows = rows
            .iter()
            .map(|row| {
                block
                    .clone()
                    .map(|column| match at(row, column) {
                        byte @ (b' ' | b'0'..=b'9') => Ok(byte),
                        other => Err(Error::parse(format!("not a digit: {}", other as char))),
                    })
                    .collect::<Result<Vec<u8>, Error>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if rows.iter().flatten().all(|byte| *byte == b' ') {
            return Err(Error::parse(format!(
                "problem at column {} has no numbers",
                block.start
            )));
        }

        Ok(Problem { rows, operator })
    }

    fn rows_as_numbers(&self) -> u128 {
        self.operator
            .apply(self.rows.iter().map(|row| number(row.iter().copied())))
    }

    fn columns_as_numbers(&self) -> u128 {
        let width = self.rows.first().map_or(0, Vec::len);

        self.operator.apply(
            (0..width)
                .rev()
                .map(|column| number(self.rows.iter().map(|row| row[column]))),
        )
    }
}

fn number(bytes: impl Iterator<Item = u8>) -> Option<u128> {
    bytes
        .filter(|byte| *byte != b' ')
        .fold(None, |value, digit| {
            Some(value.unwrap_or(0) * 10 + u128::from(digit - b'0'))
        })
}

impl Operator {
    fn apply(self, values: impl Iterator<Item = Option<u128>>) -> u128 {
        let values = values.flatten();

        match self {
            Operator::Add => values.sum(),
            Operator::Multiply => values.product(),
        }
    }
}

impl Solution for Day6 {
    const DAY: u8 = 6;

    fn solve(self) -> Result<Answer, Error> {
        let part_1: u128 = self.problems.iter().map(Problem::rows_as_numbers).sum();
        let part_2: u128 = self.problems.iter().map(Problem::columns_as_numbers).sum();

        Ok(Answer::new(part_1, part_2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    const WORKSHEET: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn puzzle_sample() {
        let result = sample(WORKSHEET).parse::<Day6>().unwrap().solve().unwrap();

        assert_eq!(result.part_1, "4277556");
        assert_eq!(result.part_2, "3263827");
    }

    #[test]
    fn reads_columns_right_to_left() {
        let problems = sample(WORKSHEET).parse::<Day6>().unwrap().problems;
        let totals: Vec<u128> = problems.iter().map(Problem::columns_as_numbers).collect();

        assert_eq!(totals, [8544, 625, 3253600, 1058]);
    }

    #[test]
    fn rejects_unknown_operators() {
        assert!(sample(&["1", "-"]).parse::<Day6>().is_err());
    }

    #[test]
    fn rejects_a_problem_with_two_operators() {
        assert!(sample(&["12", "+*"]).parse::<Day6>().is_err());
    }
}
