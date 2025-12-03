use std::{fmt::Display, time::Instant};

static mut RAN: bool = true;

fn main() {
    #[cfg(feature = "day1")]
    {
        println!("Day 1");

        execute(|| aoc1::solution(aoc1::parse_input("aoc1/src/input").unwrap()).unwrap());
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
