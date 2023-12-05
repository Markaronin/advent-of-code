use std::collections::BTreeSet;

use advent_of_code_util::*;
use itertools::Itertools;

struct Card {
    // number: usize,
    my_numbers: BTreeSet<usize>,
    winning_numbers: BTreeSet<usize>,
}
impl Card {
    pub fn from_line(line: &str) -> Self {
        let line = line.split_once(": ").unwrap().1;
        let line = line.split_once(" | ").unwrap();
        let winning_numbers = line
            .0
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<BTreeSet<_>>();
        let my_numbers = line
            .1
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<BTreeSet<_>>();
        Self {
            my_numbers,
            winning_numbers,
        }
    }

    pub fn winning_numbers_count(&self) -> usize {
        self.winning_numbers.intersection(&self.my_numbers).count()
    }

    pub fn score(&self) -> usize {
        let num_matches = self.winning_numbers_count() as u32;
        if num_matches == 0 {
            0
        } else {
            2_usize.pow(num_matches - 1)
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| Card::from_line(&line))
        .collect_vec();

    let card_scores = input.iter().map(|card| card.score()).collect_vec();
    let card_winning_numbers: Vec<usize> = input
        .iter()
        .map(|card| card.winning_numbers_count())
        .collect_vec();

    let result_1 = card_scores.iter().sum::<usize>();

    let result_2 = {
        let mut card_amounts = vec![1; card_scores.len()];
        for i in 0..card_scores.len() {
            for j in i + 1..=i + card_winning_numbers[i].min(card_scores.len()) {
                card_amounts[j] += card_amounts[i];
            }
        }

        card_amounts.iter().sum()
    };

    (result_1, result_2)
}

base_aoc!(13, 30);
