use std::collections::BTreeSet;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn repeat(num: usize, times: usize) -> usize {
    num.to_string().repeat(times).parse::<usize>().unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)[0]
        .split(',')
        .map(|raw_range| {
            raw_range
                .split('-')
                .map(|raw_num| raw_num.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .map(|tup| tup.0..=tup.1)
                .unwrap()
        })
        .collect_vec();

    let highest_number = *input.iter().map(|r| r.end()).max().unwrap();

    let mut part_1 = 0;

    let mut base = 1;

    while repeat(base, 2) < highest_number {
        let dup_num = repeat(base, 2);
        if input.iter().any(|r| r.contains(&dup_num)) {
            part_1 += dup_num;
        }
        base += 1;
    }

    let mut part_2 = BTreeSet::new();
    for repeats in 2..=highest_number.to_string().len() {
        let mut base = 1;

        while repeat(base, repeats) < highest_number {
            let dup_num = repeat(base, repeats);
            if input.iter().any(|r| r.contains(&dup_num)) {
                part_2.insert(dup_num);
            }
            base += 1;
        }
    }
    let part_2 = part_2.iter().sum();

    (part_1, part_2)
}

base_aoc!(1227775554, 4174379265);
