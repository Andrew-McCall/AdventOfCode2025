use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

pub struct Day6 {
    questions: Vec<Question>,
}

struct Question {
    values: Vec<usize>,
    operator: Operator,
}

enum Operator {
    Add,
    Multiply,
}

/// The sums read down the columns, with the operator on the bottom row.
impl FromStr for Day6 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut columns: Vec<Vec<&str>> = Vec::new();

        for line in input.lines().filter(|line| !line.trim().is_empty()) {
            for (index, word) in line.split_ascii_whitespace().enumerate() {
                match columns.get_mut(index) {
                    Some(column) => column.push(word),
                    None => columns.push(vec![word]),
                }
            }
        }

        let questions = columns
            .into_iter()
            .map(Question::from_column)
            .collect::<Result<_, _>>()?;

        Ok(Day6 { questions })
    }
}

impl Question {
    fn from_column(mut column: Vec<&str>) -> Result<Self, Error> {
        let operator = column
            .pop()
            .ok_or_else(|| Error::parse("column with no operator"))?;

        let operator = match operator {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            other => return Err(Error::parse(format!("unknown operator: {other}"))),
        };

        let values = column
            .into_iter()
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| Error::parse(format!("not a number: {value}")))
            })
            .collect::<Result<_, _>>()?;

        Ok(Question { values, operator })
    }

    fn answer(&self) -> usize {
        match self.operator {
            Operator::Add => self.values.iter().sum(),
            Operator::Multiply => self.values.iter().product(),
        }
    }
}

impl Solution for Day6 {
    const DAY: u8 = 6;

    fn solve(self) -> Result<Answer, Error> {
        let part_1: usize = self.questions.iter().map(Question::answer).sum();

        Ok(Answer::new(part_1, "unsolved"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    #[test]
    fn puzzle_sample() {
        let input = sample(&["1 2", "3 4", "+ *"]);
        let result = input.parse::<Day6>().unwrap().solve().unwrap();

        assert_eq!(result.part_1, "12");
    }

    #[test]
    fn rejects_unknown_operators() {
        assert!(sample(&["1", "-"]).parse::<Day6>().is_err());
    }
}
