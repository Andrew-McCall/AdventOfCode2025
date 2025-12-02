use std::fmt;

pub fn solution(input_path: &str) -> Result<Answer, String> {
    let mut answer = Answer {
        final_number: 50,
        zero_count: 0,
        zero_passes: 0,
    };

    let file =
        std::fs::read(input_path).map_err(|e| format!("Unable to read file: {:?}", e.kind()))?;

    let input = String::from_utf8(file).map_err(|e| {
        format!(
            "Unable to parse file: Character {}",
            e.utf8_error().valid_up_to()
        )
    })?;

    // let mut debug = 0;
    for line in input.lines() {
        // debug += 1;
        // if debug > 20 {
        //     break;
        // }
        let (start, value) = parse_line(line)?;

        let prev = answer.final_number;
        match start {
            'R' => answer.final_number += value,
            'L' => answer.final_number -= value,
            _ => return Err(format!("Invalid Line (start): {line}")),
        }

        let prev_q = prev.div_euclid(100);
        let new_q = answer.final_number.div_euclid(100);
        answer.zero_passes += (new_q - prev_q).abs();
        answer.final_number = answer.final_number.rem_euclid(100);

        if answer.final_number == 0 {
            answer.zero_count += 1;
            answer.zero_passes += 1;
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
