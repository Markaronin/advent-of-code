use std::ops::RangeInclusive;

use advent_of_code_util::{base_aoc, parse::read_blocks};
use itertools::Itertools;

fn condense_ranges(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut new_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    for range in ranges {
        if let Some(last) = new_ranges.last_mut()
            && range.start() <= last.end()
        {
            *last = *last.start()..=(*range.end().max(last.end()));
        } else {
            new_ranges.push(range);
        }
    }

    new_ranges
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);
    let ranges = input[0]
        .iter()
        .map(|line| {
            let tup = line
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap();
            tup.0..=tup.1
        })
        .sorted_by_key(|r| *r.start())
        .collect_vec();

    let ranges = condense_ranges(ranges);

    let ingredient_ids = input[1]
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();

    let part_1 = ingredient_ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    let part_2 = ranges.into_iter().map(|range| range.count()).sum();

    (part_1, part_2)
}

base_aoc!(3, 14);
