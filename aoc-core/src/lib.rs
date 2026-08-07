use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod error;

pub use error::Error;

/// A day's puzzle. The implementing type *is* the parsed input: `FromStr` turns
/// the raw file into it, `solve` consumes it.
pub trait Solution: FromStr<Err = Error> + Sized {
    const DAY: u8;

    fn solve(self) -> Result<Answer, Error>;
}

pub struct Answer {
    pub part_1: String,
    pub part_2: String,
    pub note: Option<String>,
}

impl Answer {
    pub fn new(part_1: impl Display, part_2: impl Display) -> Self {
        Answer {
            part_1: part_1.to_string(),
            part_2: part_2.to_string(),
            note: None,
        }
    }

    /// Extra detail worth printing but that is not an answer, e.g. day 1's dial.
    pub fn with_note(mut self, note: impl Display) -> Self {
        self.note = Some(note.to_string());
        self
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.part_1, self.part_2)?;
        if let Some(note) = &self.note {
            write!(f, " ({note})")?;
        }
        Ok(())
    }
}

/// A day with its type erased, so the runner can keep them all in one list.
pub struct Entry {
    pub day: u8,
    solver: fn(&str) -> Result<Answer, Error>,
}

impl Entry {
    pub const fn of<S: Solution>() -> Self {
        Entry {
            day: S::DAY,
            solver: parse_and_solve::<S>,
        }
    }

    pub fn run(&self, input: &str) -> Result<Answer, Error> {
        (self.solver)(input)
    }
}

fn parse_and_solve<S: Solution>(input: &str) -> Result<Answer, Error> {
    input.parse::<S>()?.solve()
}

/// Inputs are not in the repository (Advent of Code asks people not to publish
/// them), so this is a directory the user fills in themselves. It sits next to
/// this crate rather than next to the working directory, so the runner works
/// from anywhere.
fn inputs_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("AOC_INPUTS") {
        return PathBuf::from(directory);
    }

    match Path::new(env!("CARGO_MANIFEST_DIR")).parent() {
        Some(workspace) => workspace.join("inputs"),
        None => PathBuf::from("inputs"),
    }
}

pub fn read_input(day: u8) -> Result<String, Error> {
    let path = inputs_directory().join(format!("day{day:02}.txt"));

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => {
            return Err(Error::MissingInput {
                day,
                path: show(&path),
            });
        }
        Err(cause) => {
            return Err(Error::Unreadable {
                path: show(&path),
                cause,
            });
        }
    };

    String::from_utf8(bytes).map_err(|_| Error::NotUtf8 { path: show(&path) })
}

fn show(path: &Path) -> String {
    path.display().to_string()
}

/// Turns sample lines into the shape a `FromStr` impl expects. Test helper.
pub fn sample(lines: &[&str]) -> String {
    lines.join("\n")
}
