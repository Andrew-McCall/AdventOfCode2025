use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

const ROCK: u8 = b'@';
const GONE: u8 = b'x';

pub struct Day4 {
    grid: Vec<Vec<u8>>,
}

impl FromStr for Day4 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().as_bytes().to_vec())
            .collect();

        if grid.is_empty() {
            return Err(Error::parse("empty grid"));
        }

        let width = grid[0].len();
        if grid.iter().any(|row| row.len() != width) {
            return Err(Error::parse("grid is not rectangular"));
        }

        Ok(Day4 { grid })
    }
}

impl Solution for Day4 {
    const DAY: u8 = 4;

    fn solve(mut self) -> Result<Answer, Error> {
        let mut part_1 = 0;
        let mut part_2 = 0;
        let mut passes = 0;

        loop {
            passes += 1;
            let falling = unsupported(&self.grid);

            if passes == 1 {
                part_1 = falling.len();
            }
            if falling.is_empty() {
                break;
            }
            part_2 += falling.len();

            for (x, y) in falling {
                self.grid[y][x] = GONE;
            }
        }

        Ok(Answer::new(part_1, part_2).with_note(format!("{passes} passes")))
    }
}

/// Every rock with fewer than four rocks touching it.
fn unsupported(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut falling = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == ROCK && neighbours(grid, x, y) < 4 {
                falling.push((x, y));
            }
        }
    }

    falling
}

fn neighbours(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut count = 0;

    for down in -1..=1_isize {
        for across in -1..=1_isize {
            if down == 0 && across == 0 {
                continue;
            }

            let Some(y) = y.checked_add_signed(down) else {
                continue;
            };
            let Some(x) = x.checked_add_signed(across) else {
                continue;
            };

            if grid.get(y).and_then(|row| row.get(x)) == Some(&ROCK) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_core::sample;

    fn solve(lines: &[&str]) -> Answer {
        sample(lines).parse::<Day4>().unwrap().solve().unwrap()
    }

    #[test]
    fn counts_neighbours_at_the_edge() {
        let grid = sample(&["@@", "@@"]).parse::<Day4>().unwrap().grid;
        assert_eq!(neighbours(&grid, 0, 0), 3);
    }

    #[test]
    fn puzzle_sample() {
        let result = solve(&[
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]);
        assert_eq!(result.part_1, "13");
        assert_eq!(result.part_2, "43");
        assert_eq!(result.note.as_deref(), Some("10 passes"));
    }

    #[test]
    fn one_row() {
        let result = solve(&["@@@.@.@.@@"]);
        assert_eq!(result.part_1, "7");
    }

    #[test]
    fn rejects_ragged_grids() {
        assert!(sample(&["@@", "@"]).parse::<Day4>().is_err());
    }
}
