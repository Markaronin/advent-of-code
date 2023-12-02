use advent_of_code_util::*;
use itertools::Itertools;

fn get_program_output(input_file: &str, input_file_2: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let a = input
        .into_iter()
        .map(|line| {
            let numbers = line.chars().filter(|c| c.is_numeric()).collect_vec();
            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();

    let input = read_lines(input_file_2);

    let b = input
        .into_iter()
        .map(|line| {
            let mut numbers = Vec::new();

            for i in 0..line.len() {
                if line.chars().nth(i).unwrap().is_numeric() {
                    numbers.push(line.chars().nth(i).unwrap().to_digit(10).unwrap());
                }
                if line[i..].starts_with("one") {
                    numbers.push(1);
                }
                if line[i..].starts_with("two") {
                    numbers.push(2);
                }
                if line[i..].starts_with("three") {
                    numbers.push(3);
                }
                if line[i..].starts_with("four") {
                    numbers.push(4);
                }
                if line[i..].starts_with("five") {
                    numbers.push(5);
                }
                if line[i..].starts_with("six") {
                    numbers.push(6);
                }
                if line[i..].starts_with("seven") {
                    numbers.push(7);
                }
                if line[i..].starts_with("eight") {
                    numbers.push(8);
                }
                if line[i..].starts_with("nine") {
                    numbers.push(9);
                }
            }

            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let file_path_2 = format!("{}/testinput2", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path, &file_path_2);
        assert_eq!(part_1_output, 142);
        assert_eq!(part_2_output, 281);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path, &file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
