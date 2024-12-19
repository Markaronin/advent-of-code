use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

fn number_of_ways_to_make_pattern(
    s: &str,
    patterns: &[String],
    cache: &mut BTreeMap<String, usize>,
) -> usize {
    if s.is_empty() {
        1
    } else if cache.contains_key(s) {
        *cache.get(s).unwrap()
    } else {
        let val = patterns
            .iter()
            .filter(|pattern| s.starts_with(*pattern))
            .map(|pattern| number_of_ways_to_make_pattern(&s[pattern.len()..], patterns, cache))
            .sum();
        cache.insert(s.to_string(), val);
        val
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_blocks(input_file);
    let towels = input.pop().unwrap();
    let patterns = input.pop().unwrap()[0]
        .split(", ")
        .map(|s| s.to_string())
        .collect_vec();

    let mut cache = BTreeMap::new();
    let answer_1 = towels
        .iter()
        .filter(|towel| number_of_ways_to_make_pattern(towel, &patterns, &mut cache) > 0)
        .count();

    let answer_2 = towels
        .iter()
        .map(|towel| number_of_ways_to_make_pattern(towel, &patterns, &mut cache))
        .sum();

    (answer_1, answer_2)
}

base_aoc!(6, 16);
