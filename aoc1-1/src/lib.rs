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
    let mut counter = 50_isize;
    let mut zero_land = 0;
    let mut zero_pass = 0;

    for line in &inputs {
        let (start, value): (char, isize) = parse_line(line)?;

        match start {
            'R' => {
                counter += value;
                if counter > 99 {
                    zero_pass += 1
                };
            }
            'L' => {
                counter -= value;
                if counter < 0 {
                    zero_pass += 1
                }
            }
            _ => return Err(format!("Invalid Line: {line}")),
        }

        zero_pass += value.abs() / 100;

        counter = counter.rem_euclid(100);

        if counter == 0 {
            zero_land += 1;
        }
    }

    Ok(Answer {
        final_number: counter,
        zero_land,
        zero_pass,
    })
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
    zero_land: isize,
    zero_pass: isize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Part 1:{}, Part 2:{}, Final Counter: {}",
            self.zero_land, self.zero_pass, self.final_number
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
    fn test_simple() {
        let sample: [&str; 3] = ["L50", "R50", "L50"];

        let sample = sample.iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.zero_land, 2);
        assert_eq!(result.zero_pass, 0);
    }

    #[test]
    fn test_large() {
        let sample = [
            "L50", "L400", "R400", "R99", "R14", "L82", "L82", "L82", "L82", "L82", "L113",
        ];

        let sample = sample.iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        // assert_eq!(result.final_number, 32);
        assert_eq!(result.zero_land, 3);
        assert_eq!(result.zero_pass, 16);
    }

    #[test]
    fn test_sample() {
        let sample = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let sample = sample.iter().map(|s| s.to_string()).collect();

        let result = solution(sample).unwrap();
        assert_eq!(result.final_number, 32);
        assert_eq!(result.zero_land, 3);
        assert_eq!(result.zero_pass, 6);
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

        assert_eq!(-400_i32.rem_euclid(100), 400_i32.rem_euclid(100));
    }
}
