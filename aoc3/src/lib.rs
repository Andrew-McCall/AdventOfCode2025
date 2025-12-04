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

pub fn joltage(input: &str, size: usize) -> usize {
    let mut output = String::new();
    let mut start = 0;
    while output.len() < size {
        let next = largest(
            input
                .get(start..input.len() - size + output.len() + 1)
                .unwrap(),
        );
        output.push(next.1);
        start += next.0 + 1;
    }
    output.parse().unwrap()
}

pub fn largest(input: &str) -> (usize, char) {
    let mut chars = input.chars().enumerate();
    let mut largest = chars.next().unwrap();
    for c in chars {
        if c.1 > largest.1 {
            largest = c;
        }
    }
    largest
}

pub fn solution(input: Vec<String>) -> Result<Answer, String> {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in &input {
        part_1 += joltage(line, 2);
        part_2 += joltage(line, 12);
    }

    Ok(Answer { part_1, part_2 })
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
    fn test_sample_line() {
        let sample = ["897654321111111119"];

        let sample = sample.iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 99);
        assert_eq!(result.part_2, 976543211119);
    }

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
        assert_eq!(result.part_2, 3121910778619);
    }
}
