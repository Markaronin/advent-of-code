use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let part_1_input = input
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|a| a.to_owned())
                .collect_vec()
        })
        .collect_vec();

    let mut part_1 = 0;
    for col in 0..part_1_input[0].len() {
        let col_numbers = part_1_input[..part_1_input.len() - 1]
            .iter()
            .map(|row| row[col].parse::<usize>().unwrap());

        part_1 += match part_1_input[part_1_input.len() - 1][col].as_ref() {
            "*" => col_numbers.product(),
            "+" => col_numbers.sum::<usize>(),
            _ => unreachable!(),
        }
    }

    let mut part_2 = 0;
    let mut operator: Option<char> = None;
    let mut numbers = Vec::new();

    for col in 0..input[0].len() {
        let potential_num = (0..input.len() - 1)
            .map(|i| input[i].chars().nth(col).unwrap())
            .collect::<String>();
        if operator.is_none() {
            operator = Some(input[input.len() - 1].chars().nth(col).unwrap());
        }
        if potential_num.trim().is_empty() {
            // Finish up
            part_2 += match operator.unwrap() {
                '*' => numbers.iter().product(),
                '+' => numbers.iter().sum::<usize>(),
                _ => unreachable!(),
            };
            operator = None;
            numbers = Vec::new();
        } else {
            numbers.push(potential_num.trim().parse::<usize>().unwrap())
        }
    }
    part_2 += match operator.unwrap() {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum::<usize>(),
        _ => unreachable!(),
    };

    (part_1, part_2)
}

base_aoc!(4277556, 3263827);
