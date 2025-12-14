use std::{
    cmp::Reverse,
    collections::{BTreeSet, VecDeque},
    str::FromStr,
};

use advent_of_code_util::{base_aoc, parse::read_parsed_lines};
use itertools::Itertools;

struct IndicatorLight {
    desired_configuration: usize,
    buttons: Vec<usize>,
    joltage_requirements: [u16; 10],
}
impl IndicatorLight {
    pub fn find_fewest_button_presses_to_turn_on(&self) -> usize {
        let mut added: BTreeSet<usize> = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        added.insert(0);

        while let Some((dist, val)) = queue.pop_front() {
            if val == self.desired_configuration {
                return dist;
            }
            for b in &self.buttons {
                let possible_config = val ^ b;
                if !added.contains(&possible_config) {
                    added.insert(possible_config);
                    queue.push_back((dist + 1, possible_config));
                }
            }
        }
        panic!();
    }

    fn find_fewest_button_presses_to_set_joltage_helper(
        &self,
        starting_joltage: [u16; 10],
        current_total_presses: u16,
        current_button: usize,
        buttons: &[[u16; 10]],
        best_so_far: u16,
    ) -> Option<u16> {
        if starting_joltage == self.joltage_requirements {
            Some(current_total_presses)
        } else if current_button >= buttons.len()
            || current_total_presses >= best_so_far
            || (0..10).any(|i| {
                buttons
                    .iter()
                    .skip(current_button)
                    .map(|b| b[i])
                    .sum::<u16>()
                    == 0
                    && starting_joltage[i] != self.joltage_requirements[i]
            })
        {
            None
        } else {
            // println!(
            //     "{:?} {:?} {:?} {:?} {:?} {:?}",
            //     starting_joltage,
            //     self.joltage_requirements,
            //     buttons[current_button],
            //     current_total_presses,
            //     current_button,
            //     best_so_far
            // );
            let mut new_best = best_so_far;

            let max_presses_for_this_button = (0..10)
                .filter(|&i| buttons[current_button][i] > 0)
                .map(|i| self.joltage_requirements[i] - starting_joltage[i])
                .min()
                .unwrap_or(0);

            // Reverse so we try the more constrained ones first
            for num_presses in (0..=max_presses_for_this_button).rev() {
                let possible_configuration: [u16; 10] = starting_joltage
                    .iter()
                    .zip(buttons[current_button].iter())
                    .map(|(a, b)| a + (b * num_presses))
                    .collect_vec()
                    .try_into()
                    .unwrap();
                if let Some(attempt) = self.find_fewest_button_presses_to_set_joltage_helper(
                    possible_configuration,
                    current_total_presses + num_presses,
                    current_button + 1,
                    buttons,
                    new_best,
                ) {
                    // if attempt < u16::MAX {
                    //     println!("{attempt}");
                    // }
                    new_best = new_best.min(attempt)
                }
            }
            Some(new_best)
        }
    }

    pub fn find_fewest_button_presses_to_set_joltage(&self) -> usize {
        let buttons = self
            .buttons
            .iter()
            .map(|b| {
                format!("{:010b}", b)
                    .chars()
                    .map(|c| if c == '1' { 1 } else { 0 })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .sorted_by_key(|b: &[u16; 10]| Reverse(b.iter().sum::<u16>())) // Sort least sparse vectors to the front so we can prune more aggressively towards the end
            .collect_vec();

        self.find_fewest_button_presses_to_set_joltage_helper([0; 10], 0, 0, &buttons, u16::MAX)
            .unwrap() as usize
    }
}
impl FromStr for IndicatorLight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect_vec();
        let desired_configuration = split[0][1..split[0].len() - 1]
            .chars()
            .map(|c| c == '#')
            .rev()
            .fold(0, |acc, b| (acc << 1) | (b as usize));
        let buttons = split[1..=split.len() - 2]
            .iter()
            .map(|s| {
                let mut button = 0;
                for i in s[1..s.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                {
                    button |= 1 << i;
                }
                button
            })
            .collect_vec();

        let joltage_requirements: Vec<u16> = split[split.len() - 1]
            [1..split[split.len() - 1].len() - 1]
            .split(',')
            .map(|n| n.parse().unwrap())
            .rev()
            .collect();
        let joltage_requirements: [u16; 10] =
            std::iter::repeat_n(0, 10 - joltage_requirements.len())
                .chain(joltage_requirements)
                .collect_vec()
                .try_into()
                .unwrap();

        Ok(Self {
            desired_configuration,
            buttons,
            joltage_requirements,
        })
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<IndicatorLight> = read_parsed_lines(input_file);

    let part_1 = input
        .iter()
        .map(|i| i.find_fewest_button_presses_to_turn_on())
        .sum();

    let part_2 = input
        .iter()
        .enumerate()
        .map(|(i, light)| {
            let val = light.find_fewest_button_presses_to_set_joltage();
            println!("Finished {}/{}", i + 1, input.len());
            val
        })
        .sum();

    (part_1, part_2)
}

base_aoc!(7, 33);
