use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn next_secret_number(mut num: usize) -> usize {
    num = (num ^ (num * 64)) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;
    num
}

fn nth_secret_number(mut num: usize, n: usize) -> usize {
    for _ in 0..n {
        num = next_secret_number(num);
    }
    num
}

fn most_money(input: &[usize]) -> usize {
    let mut best_so_far = 0;
    let mut cache: Vec<BTreeMap<[isize; 4], usize>> = vec![BTreeMap::new(); input.len()];

    for j in 0..input.len() {
        let mut window = [0, 0, 0, 0];
        let mut num = input[j];
        for i in 0..2000 {
            let old_price = (num % 10) as isize;
            num = next_secret_number(num);
            let new_price = (num % 10) as isize;
            window[0] = window[1];
            window[1] = window[2];
            window[2] = window[3];
            window[3] = new_price - old_price;

            if i > 3 {
                cache[j].entry(window).or_insert(new_price as usize);
            }
        }
    }

    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let window = [a, b, c, d];
                    let money_made = (0..input.len()).flat_map(|i| cache[i].get(&window)).sum();
                    best_so_far = best_so_far.max(money_made);
                }
            }
        }
    }

    best_so_far
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    let answer_1 = input.iter().map(|num| nth_secret_number(*num, 2000)).sum();
    let answer_2 = most_money(&input);

    (answer_1, answer_2)
}

base_aoc!(37990510, 23);
