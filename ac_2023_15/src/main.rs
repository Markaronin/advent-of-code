use std::collections::VecDeque;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.as_bytes() {
        v += *c as usize;
        v *= 17;
        v %= 256;
    }
    v
}

struct Lens {
    label: String,
    focal_length: usize,
}

fn focusing_power(hash_map: &[VecDeque<Lens>]) -> usize {
    hash_map
        .iter()
        .enumerate()
        .flat_map(|(box_number, b)| {
            b.iter().enumerate().map(move |(lens_number, lens)| {
                (box_number + 1) * (lens_number + 1) * lens.focal_length
            })
        })
        .sum::<usize>()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)[0]
        .split(',')
        .map(|s| s.to_string())
        .collect_vec();

    let result_1 = input.iter().map(|s| hash(s)).sum::<usize>();

    let mut hash_map: Vec<VecDeque<Lens>> = (0..256).map(|_| VecDeque::new()).collect_vec();
    for instruction in input {
        if instruction.ends_with('-') {
            let label = &instruction[..instruction.len() - 1];
            let index = hash(label);
            if let Some(sub_index) = hash_map[index].iter().position(|l| l.label == label) {
                hash_map[index]
                    .remove(sub_index)
                    .expect("This should def happen");
            }
        } else {
            let (label, focal_length) = instruction.split('=').collect_tuple().unwrap();
            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_length.parse().unwrap(),
            };
            let index = hash(label);
            if let Some(sub_index) = hash_map[index].iter().position(|l| l.label == label) {
                hash_map[index][sub_index] = lens;
            } else {
                hash_map[index].push_back(lens);
            }
        }
    }

    let result_2 = focusing_power(&hash_map);

    (result_1, result_2)
}

base_aoc!(1320, 145);
