use std::fs::read_to_string;

use advent_of_code_util::base_aoc;
use regex::Regex;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_to_string(input_file).unwrap();

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut answer_1 = 0;
    let mut answer_2 = 0;
    for cap in re.captures_iter(&input) {
        dbg!(&cap);
        if cap[0] == *"do()" {
            enabled = true;
        } else if cap[0] == *"don't()" {
            enabled = false;
        } else {
            let mult = cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap();
            answer_1 += mult;
            if enabled {
                answer_2 += mult;
            }
        }
    }

    (answer_1, answer_2)
}

base_aoc!(161, 48);
