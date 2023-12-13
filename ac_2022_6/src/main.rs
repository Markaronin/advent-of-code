use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn find_packet_start(input: &[char], num_unique: usize) -> usize {
    input
        .windows(num_unique)
        .find_position(|window| window.iter().unique().count() == window.len())
        .unwrap()
        .0
        + num_unique
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file).get(0).unwrap().chars().collect_vec();

    let result_1 = find_packet_start(&input, 4);
    let result_2 = find_packet_start(&input, 14);

    (result_1, result_2)
}

base_aoc!(7, 19);
