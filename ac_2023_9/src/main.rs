use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (isize, isize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let (result_2, result_1) = {
        input
            .iter()
            .map(|seq| {
                let mut differences = vec![seq.clone()];
                while !differences.last().unwrap().iter().all_equal() {
                    let new_differences = differences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|pair| pair[1] - pair[0])
                        .collect_vec();
                    differences.push(new_differences);
                }
                let mut next_diff = differences.pop().unwrap()[0];
                let mut prev_diff = next_diff;
                while let Some(diffs) = differences.pop() {
                    prev_diff = diffs.first().unwrap() - prev_diff;
                    next_diff += diffs.last().unwrap();
                }
                (prev_diff, next_diff)
            })
            .fold((0, 0), |(sum1, sum2), (val1, val2)| {
                (sum1 + val1, sum2 + val2)
            })
    };

    (result_1, result_2)
}

base_aoc!(114, 2);
