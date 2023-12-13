use advent_of_code_util::parse::read_lines;

fn is_valid(num: usize, slice: &[usize]) -> bool {
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if slice[i] + slice[j] == num {
                return true;
            }
        }
    }
    false
}

fn get_program_output(input_file: &str, preamble_size: usize) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut i = preamble_size;
    loop {
        if !is_valid(input[i], &input[i - preamble_size..=i]) {
            break;
        }
        i += 1;
    }
    let first_invalid_num = input[i];

    let mut sum_contiguous_set = 0;
    for i in 0..input.len() {
        let mut sum_so_far = 0;
        let mut j = i;
        while sum_so_far < first_invalid_num {
            sum_so_far += input[j];
            j += 1;
        }
        if sum_so_far == first_invalid_num && j - i >= 2 {
            let mut contiguous_set = input[i..j].iter().clone().collect::<Vec<&usize>>();
            contiguous_set.sort_unstable();
            sum_contiguous_set =
                **contiguous_set.first().unwrap() + **contiguous_set.last().unwrap();
            break;
        }
    }

    (first_invalid_num, sum_contiguous_set)
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path, 25);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path, 5);
        assert_eq!(part_1_output, 127);
        assert_eq!(part_2_output, 62);
    }
}
