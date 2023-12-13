use advent_of_code_util::{base_aoc, intersect_vectors, parse::read_lines_of_chars};
use itertools::Itertools;

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
            let common = intersect_vectors(
                [c1, c2]
                    .into_iter()
                    .map(|vec| vec.iter().collect_vec())
                    .collect_vec(),
            );
            to_priority(**common.get(0).unwrap())
        })
        .sum();

    let result_2 = input
        .chunks_exact(3)
        .map(|elves| {
            let common = intersect_vectors(elves.iter().cloned().collect_vec());
            to_priority(*common.first().unwrap())
        })
        .sum();

    (result_1, result_2)
}

base_aoc!(157, 70);
