use advent_of_code_util::{base_aoc, parse::read_lines};

fn required_fuel(mass: isize) -> isize {
    (mass / 3) - 2
}

fn recursive_required_fuel(mass: isize) -> isize {
    let mut result_2 = 0;
    let mut to_add = required_fuel(mass);
    loop {
        result_2 += to_add;
        to_add = required_fuel(to_add);
        if to_add <= 0 {
            break;
        }
    }
    result_2
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let result_1: isize = input.iter().map(|mass| required_fuel(*mass)).sum();

    let result_2: isize = input
        .iter()
        .map(|mass| recursive_required_fuel(*mass))
        .sum();

    (result_1 as usize, result_2 as usize)
}

base_aoc!(33583, 50346);
