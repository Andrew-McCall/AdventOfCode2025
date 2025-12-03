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

pub fn greedy(line: &str) -> usize {
    let chars = line.chars().enumerate();
    let mut left = chars.clone();
    let mut largest = left.next().unwrap();

    for (i, c) in left {
        if i == line.len() - 1 {
            break;
        }
        if c > largest.1 {
            largest = (i, c);
        }
    }

    let mut chars = chars.skip(largest.0 + 1);

    let mut largest_right = chars.next().unwrap();
    for (i, c) in chars {
        if c > largest_right.1 {
            largest_right = (i, c);
        }
    }

    (largest.1.to_string() + &largest_right.1.to_string())
        .parse::<usize>()
        .unwrap()
}

pub fn solution(input: Vec<String>) -> Result<Answer, String> {
    let mut part_1 = 0;

    for line in &input {
        let left = greedy(line);
        let right = greedy(line.chars().rev().collect::<String>().as_str());

        println!("{line} {left} vs {right}");

        part_1 += left.max(right);
    }

    Ok(Answer { part_1, part_2: 0 })
}

pub struct Answer {
    part_1: usize,
    part_2: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.part_1, self.part_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        let sample = sample.iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 357);
        assert_eq!(result.part_2, 0);
    }
}
