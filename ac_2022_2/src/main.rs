use advent_of_code_util::{base_aoc, parse::read_lines};

enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    pub fn individual_score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    pub fn match_result(&self, other: &Self) -> MatchResult {
        match (self, other) {
            (Move::Rock, Move::Rock) => MatchResult::Draw,
            (Move::Rock, Move::Paper) => MatchResult::Loss,
            (Move::Rock, Move::Scissors) => MatchResult::Win,
            (Move::Paper, Move::Rock) => MatchResult::Win,
            (Move::Paper, Move::Paper) => MatchResult::Draw,
            (Move::Paper, Move::Scissors) => MatchResult::Loss,
            (Move::Scissors, Move::Rock) => MatchResult::Loss,
            (Move::Scissors, Move::Paper) => MatchResult::Win,
            (Move::Scissors, Move::Scissors) => MatchResult::Draw,
        }
    }
    pub fn match_score(&self, other: &Self) -> usize {
        self.match_result(other).score()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}
impl MatchResult {
    pub fn score(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            let mut split_line = line.split_ascii_whitespace();
            (
                split_line.next().unwrap().to_string(),
                split_line.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();

    let result_1 = input
        .iter()
        .map(|line| {
            let first = match line.0.as_ref() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Unrecognized first character"),
            };
            let second = match line.1.as_ref() {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!("Unrecognized first character"),
            };
            (first, second)
        })
        .map(|game| game.1.individual_score() + game.1.match_score(&game.0))
        .sum();

    let result_2 = input
        .iter()
        .map(|line| {
            let first = match line.0.as_ref() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Unrecognized first character"),
            };
            let second = match line.1.as_ref() {
                "X" => MatchResult::Loss,
                "Y" => MatchResult::Draw,
                "Z" => MatchResult::Win,
                _ => panic!("Unrecognized first character"),
            };
            (first, second)
        })
        .map(|game| {
            let my_move = [Move::Rock, Move::Paper, Move::Scissors]
                .into_iter()
                .find(|m| m.match_result(&game.0) == game.1)
                .unwrap();

            my_move.individual_score() + my_move.match_score(&game.0)
        })
        .sum();

    (result_1, result_2)
}

base_aoc!(15, 12);
