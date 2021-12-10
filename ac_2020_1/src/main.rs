use advent_of_code_util::read_lines;

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut part_1_output = None;
    for i in input.iter() {
        for j in input.iter() {
            if i + j == 2020 {
                part_1_output = Some(i * j);
            }
        }
    }
    let mut part_2_output = None;
    for i in input.iter() {
        for j in input.iter() {
            for k in input.iter() {
                if i + j + k == 2020 {
                    part_2_output = Some(i * j * k);
                }
            }
        }
    }
    (part_1_output.unwrap(), part_2_output.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 514579);
        assert_eq!(part_2_output, 241861950);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("2 entries that sum to 2020 multiplied: {}", part_1_output);
    println!("3 entries that sum to 2020 multiplied: {}", part_2_output);
}
