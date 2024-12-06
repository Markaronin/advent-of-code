use std::cmp::Ordering;

use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);
    let orderings = input[0]
        .iter()
        .map(|line| {
            let s = line.split_once("|").unwrap();
            (s.0.parse::<usize>().unwrap(), s.1.parse::<usize>().unwrap())
        })
        .collect_vec();

    let mut answer_1 = 0;
    let mut answer_2 = 0;

    for update in input[1].iter().map(|line| {
        line.split(",")
            .map(|u| u.parse::<usize>().unwrap())
            .collect_vec()
    }) {
        let mut new_update = update.clone();

        new_update.sort_by(|a, b| {
            if orderings.contains(&(*a, *b)) {
                Ordering::Less
            } else if orderings.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let mid_point = new_update[update.len() / 2];
        if new_update == update {
            answer_1 += mid_point;
        } else {
            answer_2 += mid_point;
        }
    }

    (answer_1, answer_2)
}

base_aoc!(143, 123);
