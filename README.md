# AdventOfCode2025

Advent of Code 2025, in Rust. Merry Christmas.

## Running

Inputs are not in the repository. Save yours as `inputs/day01.txt`, `inputs/day02.txt`
and so on, or point `AOC_INPUTS` at a directory holding them. To download them, put
your adventofcode.com session cookie in `.aoc-session` and run:

```sh
./scripts/fetch-inputs.sh        # every day still missing
./scripts/fetch-inputs.sh 2 3    # just those days
```

```sh
cargo run --release             # every day
cargo run --release -- 4        # just day 4
cargo run --release -- 1,2,3    # a list
cargo run --release -- 1..5,8   # ranges too, commas or spaces between
cargo test                      # sample answers for every day
```

Ranges mean what they do in Rust: `1..5` is days 1 to 4, `1..=5` is days 1 to 5. Days
run in order however you list them, and asking for a day that has no crate is an error
rather than a silent skip.

## Layout

| Crate      | What it is                                                       |
| ---------- | ---------------------------------------------------------------- |
| `aoc-core` | The `Solution` trait, `Answer`, the `error!` macro, input reading |
| `aoc`      | The runner: a list of days, a timer, and a `main`                |
| `aocN`     | Day N                                                            |
| `template` | Copy of a day to start from                                      |

A day is one struct that *is* the parsed input. `FromStr` builds it, `Solution::solve`
consumes it:

```rust
pub struct Day5 { ranges: Vec<(usize, usize)>, ids: Vec<usize> }

impl FromStr for Day5 { /* reject anything the solve shouldn't have to think about */ }

impl Solution for Day5 {
    const DAY: u8 = 5;
    fn solve(self) -> Result<Answer, Error> { .. }
}
```

Everything is std only, errors included — `aoc_core::error!` is a small `macro_rules!`
that writes the enum, its `Display` and its `Error` impl from one message per variant.

## Adding a day

```sh
cp -r template aoc8
```

Rename the package in `aoc8/Cargo.toml` and the struct in `aoc8/src/lib.rs`, set
`const DAY: u8 = 8`, then add it to `aoc/Cargo.toml` and to `DAYS` in `aoc/src/main.rs`.

## Progress

| Day | Part 1 | Part 2 |
| --- | ------ | ------ |
| 1   | done   | done   |
| 2   | done   | done   |
| 3   | done   | done   |
| 4   | done   | done   |
| 5   | done   | done   |
| 6   | done   | done   |
| 7   |        |        |
