use std::collections::HashMap;

use advent_of_code_util::*;
use itertools::Itertools;

struct PairInsertionRule {
    from: [char; 2],
    to: char,
}
impl PairInsertionRule {
    fn from_str(string: &str) -> Self {
        let (from_str, to_str) = string
            .split(" -> ")
            .collect_tuple::<(&str, &str)>()
            .unwrap();
        PairInsertionRule {
            from: [
                from_str.chars().nth(0).unwrap(),
                from_str.chars().nth(1).unwrap(),
            ],
            to: to_str.chars().next().unwrap(),
        }
    }
}

struct Polymerization {
    pairs: HashMap<[char; 2], usize>,
    amounts: HashMap<char, usize>,
    pair_insertion_rules: Vec<PairInsertionRule>,
}
impl Polymerization {
    fn from_blocks(blocks: Vec<Vec<String>>) -> Self {
        Polymerization {
            pairs: {
                let pairs_string = blocks[0][0].clone();
                let mut pairs = HashMap::new();
                for i in 0..pairs_string.len() - 1 {
                    let pair = [
                        pairs_string.chars().nth(i).unwrap(),
                        pairs_string.chars().nth(i + 1).unwrap(),
                    ];
                    *pairs.entry(pair).or_insert(0) += 1;
                }
                pairs
            },
            amounts: blocks[0][0].clone().chars().counts(),
            pair_insertion_rules: blocks[1]
                .clone()
                .into_iter()
                .map(|pir_string| PairInsertionRule::from_str(&pir_string))
                .collect::<Vec<_>>(),
        }
    }

    fn most_common_element(&self) -> (char, usize) {
        self.amounts
            .clone()
            .into_iter()
            .max_by_key(|x| x.1)
            .unwrap()
    }
    fn least_common_element(&self) -> (char, usize) {
        self.amounts
            .clone()
            .into_iter()
            .min_by_key(|x| x.1)
            .unwrap()
    }

    fn step(&mut self) {
        let mut new_pairs = HashMap::new();
        for rule in &self.pair_insertion_rules {
            self.pairs.get(&rule.from).iter().for_each(|pair_amount| {
                let new_pair_1 = [rule.from[0].clone(), rule.to.clone()];
                let new_pair_2 = [rule.to.clone(), rule.from[1].clone()];
                *new_pairs.entry(new_pair_1).or_insert(0) += **pair_amount;
                *new_pairs.entry(new_pair_2).or_insert(0) += **pair_amount;
                *self.amounts.entry(rule.to).or_insert(0) += **pair_amount;
            });
        }

        self.pairs = new_pairs;
    }

    fn n_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);
    let mut polymerization = Polymerization::from_blocks(input);

    polymerization.n_steps(10);

    let part_1_output =
        polymerization.most_common_element().1 - polymerization.least_common_element().1;

    polymerization.n_steps(40 - 10);

    let part_2_output =
        polymerization.most_common_element().1 - polymerization.least_common_element().1;

    (part_1_output, part_2_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 1588);
        assert_eq!(part_2_output, 2188189693529);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
