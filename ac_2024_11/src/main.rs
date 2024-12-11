use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn split_number_if_len_is_even(num: usize) -> Option<(usize, usize)> {
    let stringified = num.to_string();
    if stringified.len() % 2 == 0 {
        let mid = stringified.len() / 2;
        let first_half = stringified[..mid].parse::<usize>().unwrap();
        let second_half = stringified[mid..].parse::<usize>().unwrap();
        Some((first_half, second_half))
    } else {
        None
    }
}

fn get_number_of_stones(
    cache: &mut BTreeMap<(usize, usize), usize>,
    stone_number: usize,
    blinks_left: usize,
) -> usize {
    if let Some(num_stones) = cache.get(&(blinks_left, stone_number)) {
        *num_stones
    } else if blinks_left == 0 {
        return 1;
    } else {
        let answer = if stone_number == 0 {
            get_number_of_stones(cache, 1, blinks_left - 1)
        } else if let Some((a, b)) = split_number_if_len_is_even(stone_number) {
            get_number_of_stones(cache, a, blinks_left - 1)
                + get_number_of_stones(cache, b, blinks_left - 1)
        } else {
            get_number_of_stones(cache, stone_number * 2024, blinks_left - 1)
        };
        cache.insert((blinks_left, stone_number), answer);
        answer
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)[0]
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    // Map from blinks left, number -> number of stones
    let mut cache: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    let answer_1 = input
        .iter()
        .map(|stone_number| get_number_of_stones(&mut cache, *stone_number, 25))
        .sum::<usize>();

    let answer_2 = input
        .iter()
        .map(|stone_number| get_number_of_stones(&mut cache, *stone_number, 75))
        .sum::<usize>();

    (answer_1, answer_2)
}

base_aoc!(55312, 65601038650482);
