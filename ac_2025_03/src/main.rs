use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars};

fn find_largest_joltage_helper(
    bank: &[char],
    n: usize,
    i: usize,
    cache: &mut BTreeMap<(usize, usize), String>,
) -> String {
    if n == 0 {
        "".to_string()
    } else if bank.len() - i == n {
        bank[i..].iter().collect::<String>()
    } else if let Some(result) = cache.get(&(n, i)) {
        result.clone()
    } else {
        let with_ith_digit =
            bank[i].to_string() + &find_largest_joltage_helper(bank, n - 1, i + 1, cache);
        let without_ith_digit = find_largest_joltage_helper(bank, n, i + 1, cache);
        let result = with_ith_digit.max(without_ith_digit);
        cache.insert((n, i), result.clone());
        result
    }
}

fn find_largest_joltage(bank: &[char], n: usize) -> usize {
    let mut cache = BTreeMap::new();
    find_largest_joltage_helper(bank, n, 0, &mut cache)
        .parse()
        .unwrap()
    // let mut highest = 0;
    // for (i, left) in bank.iter().enumerate() {
    //     for right in bank.iter().skip(i + 1) {
    //         let val = (left.to_string() + &right.to_string())
    //             .parse::<usize>()
    //             .unwrap();
    //         highest = highest.max(val);
    //     }
    // }
    // highest
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let mut part_1 = 0;
    let mut part_2 = 0;
    for bank in input {
        part_1 += find_largest_joltage(&bank, 2);
        part_2 += find_largest_joltage(&bank, 12);
    }

    (part_1, part_2)
}

base_aoc!(357, 3121910778619);
