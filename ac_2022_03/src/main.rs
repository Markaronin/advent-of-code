use std::collections::BTreeSet;

use advent_of_code_util::*;

fn to_priority(c: char) -> usize {
    match c.is_lowercase() {
        true => c as usize - 96,
        false => c as usize - 38,
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let result_1 = input
        .iter()
        .map(|rucksack| {
            let (c1, c2) = rucksack.split_at(rucksack.len() / 2);
            let s1 = BTreeSet::from_iter(c1);
            let s2 = BTreeSet::from_iter(c2);
            let shared = **s1.intersection(&s2).next().unwrap();
            println!("{}", to_priority(shared));
            to_priority(shared)
        })
        .sum();

    let result_2 = input
        .chunks_exact(3)
        .map(|elves| {
            let c1 = BTreeSet::from_iter(elves[0].clone());
            let c2 = BTreeSet::from_iter(elves[1].clone());
            let c3 = BTreeSet::from_iter(elves[2].clone());
            let first_common = c1.intersection(&c2).cloned().collect::<BTreeSet<_>>();
            let common = first_common.intersection(&c3).next().unwrap();
            to_priority(*common)
        })
        .sum();

    (result_1, result_2)
}

base_aoc!(157, 70);
