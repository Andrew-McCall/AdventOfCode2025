use std::str::FromStr;

use aoc_core::{Answer, Error, Solution};

#[derive(PartialEq, Clone, Copy)]
enum CellValue {
    Beam,
    Splitter,
    Empty,
}

impl From<char> for CellValue {
    fn from(c: char) -> Self {
        match c {
            'S' | '|' => Self::Beam,
            '^' => Self::Splitter,
            _ => Self::Empty,
        }
    }
}

pub struct Day7 {
    cells: Vec<Vec<CellValue>>,
}

impl Day7 {
    pub fn part_1(&mut self) -> usize {
        let mut splits = 0;
        for y in 0..self.cells.len() - 1 {
            let row = self.cells[y].clone();
            for x in 0..row.len() {
                if row[x] == CellValue::Beam {
                    if self.cells[y + 1][x] == CellValue::Splitter {
                        splits += 1;
                        if x > 0 {
                            self.cells[y + 1][x - 1] = CellValue::Beam
                        }
                        if x < row.len() - 1 {
                            self.cells[y + 1][x + 1] = CellValue::Beam
                        }
                    } else {
                        self.cells[y + 1][x] = CellValue::Beam
                    }
                }
            }
        }
        splits
    }
}

impl FromStr for Day7 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        let mut cells = Vec::new();
        for row in input.lines() {
            let mut row_vec = Vec::new();
            for cell in row.chars().map(|c| c.into()) {
                row_vec.push(cell);
            }
            cells.push(row_vec);
        }

        Ok(Day7 { cells })
    }
}

impl Solution for Day7 {
    const DAY: u8 = 7;

    fn solve(mut self) -> Result<Answer, Error> {
        Ok(Answer::new(self.part_1(), "unsolved"))
    }
}
