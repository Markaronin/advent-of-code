use advent_of_code_util::parse::read_lines;
use itertools::Itertools;

fn run_program(instructions: &[(String, isize)]) -> Result<isize, isize> {
    let mut have_executed_instructions = vec![false; instructions.len()];
    let mut instruction_pointer = 0;
    let mut accumulator = 0;
    loop {
        if instruction_pointer == instructions.len() {
            break Ok(accumulator);
        }
        if have_executed_instructions[instruction_pointer] {
            break Err(accumulator);
        }
        have_executed_instructions[instruction_pointer] = true;
        match (
            instructions[instruction_pointer].0.as_str(), // Why must I suffer
            instructions[instruction_pointer].1,
        ) {
            ("nop", _) => instruction_pointer += 1,
            ("jmp", val) => instruction_pointer = (instruction_pointer as isize + val) as usize,
            ("acc", val) => {
                accumulator += val;
                instruction_pointer += 1;
            }
            _ => panic!(),
        }
    }
}

fn get_program_output(input_file: &str) -> (isize, isize) {
    let mut instructions = read_lines(input_file)
        .into_iter()
        .map(|line| {
            line.clone()
                .split_whitespace()
                .map(|string| string.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .map(|(instruction, amount_string)| (instruction, amount_string.parse::<isize>().unwrap()))
        .collect::<Vec<(String, isize)>>();

    let original_error_value = run_program(&instructions).unwrap_err();

    let mut result = Err(0);
    for i in 0..instructions.len() {
        if instructions[i].0 == "nop" {
            instructions[i].0 = "jmp".to_string();
            result = run_program(&instructions);
            instructions[i].0 = "nop".to_string();
        } else if instructions[i].0 == "jmp" {
            instructions[i].0 = "nop".to_string();
            result = run_program(&instructions);
            instructions[i].0 = "jmp".to_string();
        }

        if result.is_ok() {
            break;
        }
    }

    (original_error_value, result.unwrap())
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
        assert_eq!(part_1_output, 5);
        assert_eq!(part_2_output, 8);
    }
}
