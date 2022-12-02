#![allow(unused_variables, unused_mut)]
use std::collections::HashMap;

use advent_of_code_util::*;

struct ConnectionRouteFinder {
    target_joltage: usize,
    cache: HashMap<(usize, Vec<usize>), usize>,
    all_adapters: Vec<usize>,
}
impl ConnectionRouteFinder {
    fn new(target_joltage: usize, all_adapters: Vec<usize>) -> Self {
        ConnectionRouteFinder {
            target_joltage,
            cache: HashMap::new(),
            all_adapters,
        }
    }

    fn get_num_connection_routes(&mut self, starting_jolts: usize) -> usize {
        self.get_num_connection_routes_recursive(starting_jolts, self.all_adapters.clone())
    }
    fn get_num_connection_routes_recursive(
        &mut self,
        current_jolts: usize,
        remaining_adapters: Vec<usize>,
    ) -> usize {
        let key = (current_jolts, remaining_adapters.clone());
        if self.cache.contains_key(&key) {
            println!("Cache hit");
            self.cache.get(&key).unwrap().clone()
        } else {
            let val = {
                if current_jolts == self.target_joltage {
                    1
                } else if remaining_adapters.len() == 0 || {
                    let mut max_diff = usize::MAX;
                    let prev = remaining_adapters[0];
                    for i in 0..remaining_adapters.len() - 1 {}
                    max_diff > 3
                } {
                    0
                } else {
                    remaining_adapters
                        .clone()
                        .into_iter()
                        .filter(|adapter| *adapter > current_jolts && adapter - current_jolts <= 3)
                        .map(|adapter| {
                            self.get_num_connection_routes_recursive(adapter, {
                                let mut new_remaining_adapters = remaining_adapters
                                    .clone()
                                    .into_iter()
                                    .filter(|remaining_adapter| *remaining_adapter != adapter)
                                    .collect::<Vec<usize>>();
                                new_remaining_adapters.sort_unstable();
                                new_remaining_adapters
                            })
                        })
                        .sum()
                }
            };
            println!("Cache miss with {:?}", key);
            if self.cache.insert(key, val).is_some() {
                panic!();
            };
            val
        }
    }

    fn get_first_connection_route(&self, starting_jolts: usize) -> Option<(usize, usize)> {
        self.get_first_connection_route_recursive(starting_jolts, self.all_adapters.clone())
    }

    fn get_first_connection_route_recursive(
        &self,
        current_jolts: usize,
        remaining_adapters: Vec<usize>,
    ) -> Option<(usize, usize)> {
        if remaining_adapters.len() == 0 && current_jolts == self.target_joltage {
            Some((0, 0))
        } else if remaining_adapters.len() == 0 {
            None
        } else {
            remaining_adapters
                .clone()
                .into_iter()
                .filter(|adapter| *adapter > current_jolts && adapter - current_jolts <= 3)
                .find_map(|adapter| {
                    self.get_first_connection_route_recursive(adapter, {
                        let mut new_remaining_adapters = remaining_adapters
                            .clone()
                            .into_iter()
                            .filter(|remaining_adapter| *remaining_adapter != adapter)
                            .collect::<Vec<usize>>();
                        new_remaining_adapters.sort_unstable();
                        new_remaining_adapters
                    })
                    .map(|(num_differences_of_1, num_differences_of_3)| {
                        let diff = abs_diff(adapter, current_jolts);
                        match diff {
                            1 => (num_differences_of_1 + 1, num_differences_of_3),
                            3 => (num_differences_of_1, num_differences_of_3 + 1),
                            _ => (num_differences_of_1, num_differences_of_3),
                        }
                    })
                })
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut adapters = read_lines(input_file)
        .into_iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let my_adapter = adapters.iter().max().unwrap() + 3;
    adapters.push(my_adapter);

    let mut crf = ConnectionRouteFinder::new(my_adapter, adapters);

    let (num_differences_of_1, num_differences_of_3) = crf.get_first_connection_route(0).unwrap();
    let num_connection_routes = crf.get_num_connection_routes(0);

    (
        num_differences_of_1 * num_differences_of_3,
        num_connection_routes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 220);
        assert_eq!(part_2_output, 19208);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
