use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in read_lines(input_file) {
        let (first, second) = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        list1.push(first);
        list2.push(second);
    }
    list1.sort();
    list2.sort();

    let mut answer_1 = 0;
    for i in 0..list1.len() {
        answer_1 += list1[i].abs_diff(list2[i])
    }

    let mut occurances = BTreeMap::new();
    for val in list2 {
        *occurances.entry(val).or_insert(0) += 1;
    }

    let mut answer_2 = 0;
    for val in list1 {
        answer_2 += val * occurances.get(&val).unwrap_or(&0);
    }

    (answer_1, answer_2)
}

base_aoc!(11, 31);
