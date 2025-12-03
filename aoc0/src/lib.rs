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

pub fn solution(_: Vec<String>) -> Result<Answer, String> {
    Ok(Answer {})
}

pub struct Answer {}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
