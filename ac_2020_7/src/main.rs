use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use advent_of_code_util::parse::read_lines;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let bag_types = input
        .iter()
        .map(|line| {
            line.split(" bags contain ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(color, other_bags)| {
            (
                color.to_string(),
                match other_bags {
                    "no other bags." => vec![],
                    val => val
                        .split(",")
                        .map(|other_bag| {
                            other_bag
                                .split_whitespace()
                                .collect_tuple::<(&str, &str, &str, &str)>()
                                .unwrap()
                        })
                        .map(|(amount_string, color_1, color_2, _)| {
                            (
                                amount_string.parse::<usize>().unwrap(),
                                color_1.to_string() + " " + color_2,
                            )
                        })
                        .collect::<Vec<(usize, String)>>(),
                },
            )
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();

    let mut inner_bag_counts: HashMap<String, usize> = HashMap::default();
    fn get_inner_bag_count(
        inner_bag_counts: &mut HashMap<String, usize>,
        bag_types: &HashMap<String, Vec<(usize, String)>>,
        bag_type: String,
    ) -> usize {
        match inner_bag_counts.get(&bag_type).map(|entry| entry.clone()) {
            Some(result) => result,
            None => bag_types
                .get(&bag_type)
                .unwrap()
                .iter()
                .map(|(amount, inner_bag_type)| {
                    amount
                        * (get_inner_bag_count(inner_bag_counts, bag_types, inner_bag_type.clone())
                            + 1)
                })
                .sum(),
        }
    }

    let starting_bag = "shiny gold";
    let mut all_bags: HashSet<String> = HashSet::default();
    let mut bag_queue = bag_types
        .iter()
        .filter(|bag_type| {
            bag_type
                .1
                .iter()
                .any(|(_, sub_bag_type)| sub_bag_type == starting_bag)
        })
        .map(|bag_type| bag_type.0.clone())
        .collect::<Vec<String>>();

    while bag_queue.len() > 0 {
        let bag_to_process = bag_queue.pop().unwrap();

        bag_queue.append(
            bag_types
                .iter()
                .filter(|bag_type| {
                    !bag_queue.contains(bag_type.0) && !all_bags.contains(bag_type.0)
                })
                .filter(|bag_type| {
                    bag_type
                        .1
                        .iter()
                        .any(|(_, sub_bag_type)| *sub_bag_type == bag_to_process)
                })
                .map(|bag_type| bag_type.0.clone())
                .collect::<Vec<String>>()
                .as_mut(),
        );

        all_bags.insert(bag_to_process);
    }

    (
        all_bags.len(),
        get_inner_bag_count(&mut inner_bag_counts, &bag_types, starting_bag.to_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 4);
        assert_eq!(part_2_output, 32);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
