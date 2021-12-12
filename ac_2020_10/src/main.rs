use advent_of_code_util::*;

fn get_connection_route(
    current_jolts: usize,
    target_joltage: usize,
    remaining_adapters: Vec<usize>,
) -> Option<(usize, usize)> {
    if remaining_adapters.len() == 0 && current_jolts == target_joltage {
        Some((0, 0))
    } else if remaining_adapters.len() == 0 {
        None
    } else {
        remaining_adapters
            .clone()
            .into_iter()
            .filter(|adapter| *adapter > current_jolts && adapter - current_jolts <= 3)
            .find_map(|adapter| {
                get_connection_route(
                    adapter,
                    target_joltage,
                    remaining_adapters
                        .clone()
                        .into_iter()
                        .filter(|remaining_adapter| *remaining_adapter != adapter)
                        .collect::<Vec<usize>>(),
                )
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

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut adapters = read_lines(input_file)
        .into_iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let my_adapter = adapters.iter().max().unwrap() + 3;
    adapters.push(my_adapter);

    let (num_differences_of_1, num_differences_of_3) =
        get_connection_route(0, my_adapter, adapters).unwrap();

    (num_differences_of_1 * num_differences_of_3, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 220);
        assert_eq!(part_2_output, 0);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
