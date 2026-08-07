use std::process::ExitCode;
use std::time::Instant;

use aoc_core::{Entry, read_input};

const DAYS: &[Entry] = &[
    Entry::of::<aoc1::Day1>(),
    Entry::of::<aoc2::Day2>(),
    Entry::of::<aoc3::Day3>(),
    Entry::of::<aoc4::Day4>(),
    Entry::of::<aoc5::Day5>(),
    Entry::of::<aoc6::Day6>(),
    Entry::of::<aoc7::Day7>(),
];

const USAGE: &str = "usage: aoc [days]
  days   a list like 4, 1,2,3 or 1..5,8; empty runs every day
  ranges follow Rust, so 1..5 is 1 to 4 and 1..=5 is 1 to 5";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let days = match choose(&arguments) {
        Ok(days) => days,
        Err(problem) => {
            eprintln!("{problem}\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    let mut everything_worked = true;

    for entry in days {
        println!("Day {}", entry.day);
        everything_worked &= report(entry);
    }

    if everything_worked {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// The days the arguments asked for, in day order. No arguments means all of them.
fn choose(arguments: &[String]) -> Result<Vec<&'static Entry>, String> {
    if arguments.is_empty() {
        return Ok(DAYS.iter().collect());
    }

    let wanted = parse_days(arguments)?;

    if let Some(day) = wanted
        .iter()
        .find(|day| !DAYS.iter().any(|e| e.day == **day))
    {
        return Err(format!("no solution for day {day}"));
    }

    let days: Vec<&Entry> = DAYS
        .iter()
        .filter(|entry| wanted.contains(&entry.day))
        .collect();

    if days.is_empty() {
        return Err("no days selected".to_string());
    }

    Ok(days)
}

/// Splits the arguments on commas and spaces, expanding any ranges as it goes.
fn parse_days(arguments: &[String]) -> Result<Vec<u8>, String> {
    let mut days = Vec::new();

    for piece in arguments
        .iter()
        .flat_map(|argument| argument.split([',', ' ']))
        .filter(|piece| !piece.is_empty())
    {
        days.extend(parse_piece(piece)?);
    }

    Ok(days)
}

fn parse_piece(piece: &str) -> Result<Vec<u8>, String> {
    let Some((start, end)) = piece.split_once("..") else {
        return Ok(vec![parse_day(piece)?]);
    };

    let (end, inclusive) = match end.strip_prefix('=') {
        Some(end) => (end, true),
        None => (end, false),
    };

    let start = parse_day(start)?;
    let end = parse_day(end)?;

    if end < start {
        return Err(format!("backwards range: {piece}"));
    }

    if inclusive {
        Ok((start..=end).collect())
    } else {
        Ok((start..end).collect())
    }
}

fn parse_day(text: &str) -> Result<u8, String> {
    text.trim()
        .parse()
        .map_err(|_| format!("not a day: {text}"))
}

fn report(entry: &Entry) -> bool {
    let input = match read_input(entry.day) {
        Ok(input) => input,
        Err(problem) => {
            println!("  {problem}\n");
            return false;
        }
    };

    let start = Instant::now();
    let answer = entry.run(&input);
    let elapsed = start.elapsed();

    match answer {
        Ok(answer) => {
            println!("  {answer}");
            println!("  took {elapsed:?}\n");
            true
        }
        Err(problem) => {
            println!("  {problem}\n");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn days(arguments: &[&str]) -> Result<Vec<u8>, String> {
        let arguments: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();
        parse_days(&arguments)
    }

    fn chosen(arguments: &[&str]) -> Result<Vec<u8>, String> {
        let arguments: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();
        choose(&arguments).map(|days| days.iter().map(|entry| entry.day).collect())
    }

    #[test]
    fn reads_one_day() {
        assert_eq!(days(&["4"]), Ok(vec![4]));
    }

    #[test]
    fn reads_a_list() {
        assert_eq!(days(&["1,2,3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn reads_ranges_the_way_rust_does() {
        assert_eq!(days(&["1..5"]), Ok(vec![1, 2, 3, 4]));
        assert_eq!(days(&["1..=5"]), Ok(vec![1, 2, 3, 4, 5]));
        assert_eq!(days(&["1..5,8"]), Ok(vec![1, 2, 3, 4, 8]));
    }

    #[test]
    fn separates_on_commas_and_spaces() {
        assert_eq!(days(&["1,2, 3"]), Ok(vec![1, 2, 3]));
        assert_eq!(days(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn complains_about_junk() {
        assert!(days(&["nine"]).is_err());
        assert!(days(&["1..x"]).is_err());
        assert!(days(&["5..1"]).is_err());
    }

    #[test]
    fn keeps_day_order_and_drops_repeats() {
        assert_eq!(chosen(&["3,1,1"]), Ok(vec![1, 3]));
    }

    #[test]
    fn no_arguments_runs_everything() {
        assert_eq!(chosen(&[]), Ok(vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn complains_about_days_that_do_not_exist() {
        assert!(chosen(&["9"]).is_err());
        assert!(chosen(&["3..3"]).is_err());
    }
}
