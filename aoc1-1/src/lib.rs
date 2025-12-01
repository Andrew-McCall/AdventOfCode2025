use std::fmt;

pub fn solution(input_path: &str) -> Result<Answer, String> {
    println!("Hello, world!");

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

    for line in input.lines() {
        let (start, value) = parse_line(line)?;

        match start {
            'R' => answer.final_number += value,
            'L' => answer.final_number -= value,
            _ => return Err(format!("Invalid Line (start): {line}")),
        }

        let new_zero_passes = answer.final_number / 100;
        answer.zero_passes += (new_zero_passes - 1).abs();
        answer.final_number += 100;
        answer.final_number %= 100;

        print!(
            "{line} {} ({})",
            answer.final_number,
            (new_zero_passes - 1).abs()
        );

        if answer.final_number == 0 {
            answer.zero_count += 1;
        }
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
