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

pub fn solution(input: Vec<String>) -> Result<Answer, String> {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in input {
        if line.is_empty() {
            continue;
        }

        let mut range_split = line.split("-");
        let first = range_split.next().unwrap();
        let first: usize = first.parse().unwrap();
        if let Some(second) = range_split.next() {
            let second: usize = second.parse().unwrap();
            ranges.push((first, second));
        } else {
            ids.push(first);
        }
    }

    let ranges = merge_ranges(ranges);

    let part_1 = ids.iter().filter(|id| is_fresh(&ranges, **id)).count();
    let part_2 = ranges.iter().map(|(start, end)| end - start + 1).sum();

    Ok(Answer { part_1, part_2 })
}

pub fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

fn is_fresh(ranges: &[(usize, usize)], id: usize) -> bool {
    for (start, end) in ranges {
        if &id >= start && &id <= end {
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
        write!(f, "Part 1: {}, Part 2: {}", self.part_1, self.part_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = vec!["3-5", "10-14", "16-20", "12-18", "10-20", "1", "2", "3"]
            .into_iter()
            .map(str::to_string)
            .collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.part_1, 1);
        assert_eq!(result.part_2, 14);
    }
}
