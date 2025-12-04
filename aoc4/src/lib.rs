use std::fmt;

pub fn parse_input(input_path: &str) -> Option<Vec<String>> {
    let file =
        std::fs::read(input_path).unwrap_or_else(|_| panic!("Failed to read file: {input_path}"));

    Some(
        String::from_utf8(file)
            .unwrap_or_else(|_| panic!("Failed to parse file: {input_path}"))
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect(),
    )
}

pub enum Slot {
    Empty,
    Paper,
}

pub fn solution(input: Vec<String>) -> Result<Answer, String> {
    let mut part_1 = 0;

    let w = input.first().unwrap().len() as i32;
    let h = input.len() as i32;

    for y in 0..h {
        for x in 0..w {
            let my = y as usize;
            let mx = x as usize;

            if let Some(row) = input.get(my)
                && let Some(space) = row.get(mx..mx + 1)
                && space == "@"
            {
                let mut adjacent = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if j == 0 && i == 0 {
                            continue;
                        }

                        let y: i32 = y + j;
                        let x: i32 = x + i;

                        if x < 0 || y < 0 || y > h || x > w {
                            continue;
                        }

                        let x = x as usize;
                        let y = y as usize;

                        if let Some(row) = input.get(y)
                            && let Some(space) = row.get(x..x + 1)
                            && space == "@"
                        {
                            adjacent += 1;
                        }
                    }
                }

                if adjacent < 4 {
                    println!("{} {} {space} {}", x, y, adjacent);
                    part_1 += 1;
                }
            }
        }
    }

    Ok(Answer { part_1 })
}

pub struct Answer {
    part_1: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {}", self.part_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = [
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
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 13);
    }

    #[test]
    fn test_line_sample() {
        let sample = ["@@@.@.@.@@"].iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 7);
    }
}
