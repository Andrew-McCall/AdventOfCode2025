use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

const DIAL: isize = 100;
const START: isize = 50;

pub struct Day1 {
    turns: Vec<Turn>,
}

enum Turn {
    Left(isize),
    Right(isize),
}

impl FromStr for Day1 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let turns = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Turn::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Day1 { turns })
    }
}

impl FromStr for Turn {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Error> {
        let line = line.trim();
        let (direction, amount) = line
            .split_at_checked(1)
            .ok_or_else(|| Error::parse(format!("empty turn: {line:?}")))?;

        let amount = amount
            .parse()
            .map_err(|_| Error::parse(format!("not a number: {line}")))?;

        match direction {
            "L" => Ok(Turn::Left(amount)),
            "R" => Ok(Turn::Right(amount)),
            _ => Err(Error::parse(format!("unknown direction: {line}"))),
        }
    }
}

impl Solution for Day1 {
    const DAY: u8 = 1;

    fn solve(self) -> Result<Answer, Error> {
        let mut position = START;
        let mut landings = 0;
        let mut touches = 0;

        for turn in &self.turns {
            let next = match turn {
                Turn::Left(amount) => position - amount,
                Turn::Right(amount) => position + amount,
            };

            touches += zeros_crossed(position, next);
            position = next.rem_euclid(DIAL);

            if position == 0 {
                landings += 1;
            }
        }

        Ok(Answer::new(landings, touches).with_note(format!("dial on {position}")))
    }
}

/// How many times the dial reads zero moving from `from` to `to`, counting the
/// end of the move but not the start. Zero is every multiple of `DIAL`, so this
/// is just the number of multiples in the half-open interval between the two.
fn zeros_crossed(from: isize, to: isize) -> usize {
    let count = if to >= from {
        to.div_euclid(DIAL) - from.div_euclid(DIAL)
    } else {
        divide_up(from) - divide_up(to)
    };

    count as usize
}

fn divide_up(value: isize) -> isize {
    (value + DIAL - 1).div_euclid(DIAL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    fn solve(lines: &[&str]) -> Answer {
        sample(lines).parse::<Day1>().unwrap().solve().unwrap()
    }

    #[test]
    fn parses_a_turn() {
        assert!(matches!("R100".parse::<Turn>(), Ok(Turn::Right(100))));
        assert!(matches!("L100".parse::<Turn>(), Ok(Turn::Left(100))));
        assert!("D100".parse::<Turn>().is_err());
        assert!("R1x0".parse::<Turn>().is_err());
    }

    #[test]
    fn counts_zeros_in_one_move() {
        // Down from 50 to -18 passes zero once.
        assert_eq!(zeros_crossed(50, -18), 1);
        // Landing on zero counts.
        assert_eq!(zeros_crossed(52, 100), 1);
        // Leaving zero does not count it again.
        assert_eq!(zeros_crossed(0, -5), 0);
        // A long move wraps several times.
        assert_eq!(zeros_crossed(0, -400), 4);
    }

    #[test]
    fn simple() {
        let result = solve(&["L50", "R50", "L50"]);
        assert_eq!(result.part_1, "2");
        // Both landings on zero are also touches.
        assert_eq!(result.part_2, "2");
    }

    #[test]
    fn large() {
        let result = solve(&[
            "L50", "L400", "R400", "R99", "R14", "L82", "L82", "L82", "L82", "L82", "L113",
        ]);
        assert_eq!(result.part_1, "3");
        assert_eq!(result.part_2, "16");
    }

    #[test]
    fn puzzle_sample() {
        let result = solve(&[
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]);
        assert_eq!(result.part_1, "3");
        assert_eq!(result.part_2, "6");
        assert_eq!(result.note.as_deref(), Some("dial on 32"));
    }
}
