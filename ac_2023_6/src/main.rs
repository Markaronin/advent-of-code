use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);
    let times = input[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    let distances = input[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    let result_1 = times
        .iter()
        .zip_eq(distances.iter())
        .map(|(time_amt, best_distance)| {
            let mut num_better_distances = 0;

            for push_time in 0..*time_amt {
                let speed = push_time;
                let travel_time = time_amt - push_time;
                let distance = speed * travel_time;
                if distance > *best_distance {
                    num_better_distances += 1;
                }
            }
            num_better_distances
        })
        .product();

    let part_2_time = input[0]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();

    let part_2_distance = input[1]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();

    let result_2 = (0..part_2_time)
        .into_iter()
        .map(|push_time| {
            let speed = push_time;
            let travel_time = part_2_time - push_time;
            let distance = speed * travel_time;
            if distance > part_2_distance {
                1
            } else {
                0
            }
        })
        .sum();

    (result_1, result_2)
}

base_aoc!(288, 71503);
