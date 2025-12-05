use std::fmt;

pub fn solution(mut input: Vec<String>) -> Result<Answer, String> {
    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut passes = 0;

    loop {
        passes += 1;

        let removed = pass(&input);
        if passes == 1 {
            part_1 += removed.len();
        }

        if removed.is_empty() {
            break;
        }

        part_2 += removed.len();

        for (x, y) in removed {
            if let Some(row) = input.get_mut(y) {
                row.replace_range(x..x + 1, "x");
            }
        }
    }

    Ok(Answer {
        part_1,
        part_2,
        passes,
    })
}

pub fn pass(input: &[String]) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
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
                    output.push((mx, my));
                }
            }
        }
    }
    output
}

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

pub struct Answer {
    part_1: usize,
    part_2: usize,
    passes: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Part 1: {} Part 2: {} ({})",
            self.part_1, self.part_2, self.passes
        )
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
        assert_eq!(result.part_2, 43);
        assert_eq!(result.passes, 10);
    }

    #[test]
    fn test_line_sample() {
        let sample = ["@@@.@.@.@@"].iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 7);
    }
}
