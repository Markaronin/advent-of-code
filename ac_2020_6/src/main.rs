use std::collections::HashSet;

use advent_of_code_util::parse::read_blocks;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file);

    let mut union_count = 0;
    let mut intersection_count = 0;

    for block in input {
        let char_sets = block
            .iter()
            .map(|string| string.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();
        union_count += char_sets
            .clone()
            .into_iter()
            .reduce(|a, b| (&a | &b))
            .unwrap()
            .len();
        intersection_count += char_sets
            .clone()
            .into_iter()
            .reduce(|a, b| (&a & &b))
            .unwrap()
            .len();
    }

    (union_count, intersection_count)
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 11);
        assert_eq!(part_2_output, 6);
    }
}
