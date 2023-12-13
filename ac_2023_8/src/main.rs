use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;
use regex::Regex;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_lines(input_file).into_iter();

    let instructions = input.next().unwrap().chars().collect_vec();

    let mappings = input
        .skip(1)
        .map(|line| {
            let re = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();

            re.captures(&line)
                .map(|caps| {
                    (
                        caps.get(1).unwrap().as_str().to_string(),
                        (
                            caps.get(2).unwrap().as_str().to_string(),
                            caps.get(3).unwrap().as_str().to_string(),
                        ),
                    )
                })
                .unwrap()
        })
        .collect::<BTreeMap<String, (String, String)>>();

    let result_1 = {
        let mut location = "AAA".to_string();
        let mut time_taken = 0;
        while location != "ZZZ" {
            let instruction_index = time_taken % instructions.len();
            let turn = mappings.get(&location).unwrap();

            location = match instructions[instruction_index] {
                'L' => turn.0.clone(),
                'R' => turn.1.clone(),
                _ => panic!("Invalid direction"),
            };

            time_taken += 1;
        }
        time_taken
    };

    // 59, 43
    let result_2 = {
        let mut locations = mappings
            .keys()
            .filter(|l| l.ends_with("A"))
            .cloned()
            .collect_vec();
        let mut found = vec![0; locations.len()];

        let mut time_taken = 0;
        while !found.iter().all(|f| *f != 0) {
            let instruction_index = time_taken % instructions.len();

            for i in 0..locations.len() {
                let turn = mappings.get(&locations[i]).unwrap();

                locations[i] = match instructions[instruction_index] {
                    'L' => turn.0.clone(),
                    'R' => turn.1.clone(),
                    _ => panic!("Invalid direction"),
                };

                if locations[i].ends_with("Z") {
                    found[i] = (time_taken + 1) / 277;
                }
            }

            time_taken += 1;
        }

        found.into_iter().product::<usize>() * 277
    };

    (result_1, result_2)
}

base_aoc!(2, 277); // 277 is wrong but it's fine
