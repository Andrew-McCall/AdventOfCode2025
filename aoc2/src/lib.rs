use std::fmt;

pub fn parse_input(input_path: &str) -> Option<Vec<String>> {
    let file =
        std::fs::read(input_path).unwrap_or_else(|_| panic!("Failed to read file: {input_path}"));
    Some(
        String::from_utf8(file)
            .unwrap_or_else(|_| panic!("Failed to parse file: {}", input_path))
            .split(',')
            .map(str::to_string)
            .collect::<Vec<String>>(),
    )
}

pub fn solution(inputs: Vec<String>) -> Result<Answer, String> {
    assert!(inputs.len() > 1);

    let mut part_1 = 0;
    let mut part_2 = 0;
    for range in &inputs {
        let mut range_split = range.split("-");
        let range_start_str = range_split
            .next()
            .unwrap_or_else(|| panic!("Invalid Range: {range}"));
        let range_start: usize = range_start_str
            .parse()
            .unwrap_or_else(|_| panic!("Invalid Range: {range}"));
        let range_end_str = range_split
            .next()
            .unwrap_or_else(|| panic!("Invalid Range: {range}"));
        let range_end: usize = range_end_str
            .parse()
            .unwrap_or_else(|_| panic!("Invalid Range: {range}"));

        for i in range_start..=range_end {
            let i_str = &i.to_string();
            if has_repeat(i_str) {
                part_1 += i;
            }
            if has_any_repeat(i_str) {
                part_2 += i;
            }
        }
    }
    Ok(Answer { part_1, part_2 })
}

fn has_repeat(s: &str) -> bool {
    let (start, end) = s.split_at(s.len() / 2);
    if start == end {
        return true;
    }
    false
}

fn has_any_repeat(s: &str) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }
    for i in 1..=len / 2 {
        if !len.is_multiple_of(i) {
            continue;
        }
        let pattern = &s[..i];
        if s.chars()
            .collect::<Vec<_>>()
            .chunks(i)
            .all(|chunk| chunk.iter().collect::<String>() == pattern)
        {
            return true;
        }
    }
    false
}

pub struct Answer {
    part_1: usize,
    part_2: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.part_1, self.part_2)
    }
}
