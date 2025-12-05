use std::{fmt::Display, time::Instant};

static mut RAN: bool = true;

fn main() {
    #[cfg(feature = "day1")]
    {
        println!("Day One");
        execute(|| aoc1::solution(aoc1::parse_input("aoc1/src/input").unwrap()).unwrap());
    }

    #[cfg(feature = "day2")]
    {
        println!("Day Two");
        execute(|| aoc2::solution(aoc2::parse_input("aoc2/src/input").unwrap()).unwrap());
    }

    #[cfg(feature = "day3")]
    {
        println!("Day Three");
        execute(|| aoc3::solution(aoc3::parse_input("aoc3/src/input.txt").unwrap()).unwrap());
    }

    #[cfg(feature = "day4")]
    {
        println!("Day Four");
        execute(|| aoc4::solution(aoc4::parse_input("aoc4/src/input.txt").unwrap()).unwrap());
    }

    #[cfg(feature = "day5")]
    {
        println!("Day Five");
        execute(|| aoc5::solution(aoc5::parse_input("aoc5/src/input.txt").unwrap()).unwrap());
    }

    if unsafe { RAN } {
        println!("No day(s) selected.");
    }
}

fn execute<T: Display>(action: impl Fn() -> T) {
    unsafe { RAN = true }

    let start = Instant::now();
    let result = action();
    let duration = start.elapsed();

    println!("Answer: {result}");
    println!("Time elapsed: {:.8} seconds\n", duration.as_secs_f64());
}
