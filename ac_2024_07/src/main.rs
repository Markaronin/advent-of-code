use std::str::FromStr;

use advent_of_code_util::{base_aoc, parse::read_parsed_lines};
use itertools::{repeat_n, Itertools};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Combine,
}

struct Equation {
    answer: usize,
    numbers: Vec<usize>,
}
impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_line = s.split(": ");
        let answer = split_line.next().unwrap().parse::<usize>().unwrap();
        let numbers = split_line
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        Ok(Self { answer, numbers })
    }
}
impl Equation {
    fn valid_operators(&self, operators: &[Operator]) -> bool {
        assert_eq!(operators.len(), self.numbers.len() - 1);
        let mut result = self.numbers[0];
        for (number, operator) in self.numbers.iter().skip(1).zip_eq(operators.iter()) {
            match operator {
                Operator::Add => result += number,
                Operator::Multiply => result *= number,
                Operator::Combine => {
                    result = (result.to_string() + &number.to_string()).parse().unwrap()
                }
            }
        }
        result == self.answer
    }
    pub fn get_possible_operators(&self, valid_operators: Vec<Operator>) -> Option<Vec<Operator>> {
        repeat_n(valid_operators, self.numbers.len() - 1)
            .multi_cartesian_product()
            .find(|operators| self.valid_operators(operators))
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<Equation> = read_parsed_lines(input_file);

    let answer_1 = input
        .iter()
        .filter(|e| {
            e.get_possible_operators(vec![Operator::Add, Operator::Multiply])
                .is_some()
        })
        .map(|e| e.answer)
        .sum::<usize>();

    let answer_2 = input
        .iter()
        .filter(|e| {
            e.get_possible_operators(vec![Operator::Add, Operator::Multiply, Operator::Combine])
                .is_some()
        })
        .map(|e| e.answer)
        .sum::<usize>();

    (answer_1, answer_2)
}

base_aoc!(3749, 11387);
