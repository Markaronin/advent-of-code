use advent_of_code_util::{base_aoc_ignore_tests, parse::read_lines};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let part_1 = read_lines(input_file)
        .into_iter()
        .filter(|line| line.contains('x'))
        .filter(|line| {
            let mut s = line.split(": ");
            let area = {
                let (x, y) = s
                    .next()
                    .unwrap()
                    .split('x')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize)>()
                    .unwrap();
                (x / 3) * (y / 3)
            };
            let total_boxes = s
                .next()
                .unwrap()
                .split(' ')
                .map(|b| b.parse::<usize>().unwrap())
                .sum::<usize>();

            total_boxes <= area
        })
        .count();

    (part_1, 0)
}

base_aoc_ignore_tests!(2, 0);
