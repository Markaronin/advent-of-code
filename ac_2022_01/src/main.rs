use advent_of_code_util::*;
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file)
        .into_iter()
        .map(|block| {
            block
                .into_iter()
                .map(|line| line.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result_1 = input.iter().map(|block| block.iter().sum()).max().unwrap();

    let result_2 = input
        .iter()
        .map(|block| block.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>();

    (result_1, result_2)
}

base_aoc!(24000, 45000);
