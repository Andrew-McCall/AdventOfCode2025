use std::{fmt, usize};

pub fn solution(input: Vec<Vec<String>>) -> Result<Answer, String> {
    let mut part_1 = 0;
    let part_2 = 0;

    for question in &input {
        let mut question = question.clone();
        let operator = question.pop().unwrap();
        let question: Vec<usize> = question.iter().map(|s| s.parse().unwrap()).collect();
        match operator.as_str() {
            "+" => part_1 += question.iter().sum::<usize>(),
            "*" => part_1 += question.iter().product::<usize>(),
            _ => {
                panic!("Unkown Operation: {}", operator)
            }
        }
    }

    Ok(Answer { part_1, part_2 })
}

pub fn parse_input(input_path: &str) -> Vec<Vec<String>> {
    let file =
        std::fs::read(input_path).unwrap_or_else(|_| panic!("Failed to read file: {input_path}"));

    let mut questions: Vec<Vec<String>> = Vec::new();

    for line in String::from_utf8(file)
        .unwrap_or_else(|_| panic!("Failed to parse file: {input_path}"))
        .lines()
    {
        for (index, word) in line.split_ascii_whitespace().enumerate() {
            let word = word.to_string();
            if let Some(question) = questions.get_mut(index) {
                question.push(word);
            } else {
                questions.push(vec![word]);
            }
        }
    }

    questions
}

pub struct Answer {
    part_1: usize,
    part_2: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.part_1, self.part_2)
    }
}
