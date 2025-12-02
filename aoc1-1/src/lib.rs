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

pub fn solution(inputs: Vec<String>) -> Result<Answer, String> {
    let mut answer = Answer {
        final_number: 50,
        zero_count: 0,
        zero_passes: 0,
    };

    for line in inputs {
        let (start, value) = parse_line(&line)?;

        match start {
            'R' => answer.final_number += value,
            'L' => answer.final_number -= value,
            _ => return Err(format!("Invalid Line (start): {line}")),
        }

        let passes = answer.final_number.div_euclid(100).abs();
        answer.zero_passes += passes;
        answer.final_number = answer.final_number.rem_euclid(100);

        if answer.final_number == 0 {
            answer.zero_count += 1;
            if passes == 0 {
                answer.zero_passes += 1;
            }
        }

        println!("{} {}", answer.final_number, answer.zero_passes);
    }

    Ok(answer)
}

fn parse_line(line: &str) -> Result<(char, isize), String> {
    let mut chars = line.chars();

    let start = chars
        .next()
        .ok_or_else(|| format!("Invalid Line (empty): {line}"))?;

    let rest = chars.as_str();

    let value = rest
        .parse::<isize>()
        .map_err(|_| format!("Invalid Number: {line}"))?;

    Ok((start, value))
}

pub struct Answer {
    final_number: isize,
    zero_count: isize,
    zero_passes: isize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.zero_count, self.zero_passes, self.final_number
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("R100").unwrap(), ('R', 100));
        assert_eq!(parse_line("L100").unwrap(), ('L', 100));
    }

    #[test]
    fn test_div_euclid() {
        let mut counter: i32;
        counter = 100;
        counter = counter.div_euclid(100);
        assert_eq!(counter, 1);

        counter = -250;
        counter = counter.div_euclid(100);
        assert_eq!(counter, -3);

        counter = -25;
        counter = counter.div_euclid(100);
        assert_eq!(counter, -1);
    }

    #[test]
    fn test_rem_euclid() {
        let mut counter: i32;
        counter = 100;
        counter = counter.rem_euclid(100);
        assert_eq!(counter, 0);

        counter = 99;
        counter = counter.rem_euclid(100);
        assert_eq!(counter, 99);

        counter = 102;
        counter = counter.rem_euclid(100);
        assert_eq!(counter, 2);

        counter = -100;
        counter = counter.rem_euclid(100);
        assert_eq!(counter, 0);

        counter = -51;
        counter = counter.rem_euclid(100);
        assert_eq!(counter, 49);
    }
}
