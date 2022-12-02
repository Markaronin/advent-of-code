use advent_of_code_util::*;
use itertools::Itertools;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file)
        .into_iter()
        .map(|block| {
            block
                .into_iter()
                .map(|line| line.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result_1 = input.iter().map(|block| block.iter().sum()).max().unwrap();

    let result_2 = input
        .iter()
        .map(|block| block.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>();

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 24000);
        assert_eq!(part_2_output, 45000);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
