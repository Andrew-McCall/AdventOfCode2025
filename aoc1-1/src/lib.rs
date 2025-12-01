use std::fmt;

pub fn solution(input_path: &str) -> Result<Answer, String> {
    println!("Hello, world!");

    let mut answer = Answer {
        final_number: 50,
        zero_count: 0,
    };

    let file =
        std::fs::read(input_path).map_err(|e| format!("Unable to read file: {:?}", e.kind()))?;

    let input = String::from_utf8(file).map_err(|e| {
        format!(
            "Unable to parse file: Character {}",
            e.utf8_error().valid_up_to()
        )
    })?;

    let mut debug = 0;

    for line in input.lines() {
        debug += 1;
        let (start, value) = parse_line(line)?;

        // if debug > 10 {
        //     break;
        // }

        match start {
            'R' => {
                answer.final_number = (answer.final_number + value + 100) % 100;
            }
            'L' => {
                answer.final_number = (answer.final_number - value + 100) % 100;
            }
            _ => return Err(format!("Invalid Line (start): {line}")),
        }

        print!(" {line} {}", answer.final_number);

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
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.zero_count, self.final_number)
    }
}
