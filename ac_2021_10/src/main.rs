use std::collections::HashMap;

use advent_of_code_util::parse::read_lines;

fn find_first_illegal_character(line: &str) -> Option<char> {
    let bracket_close_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut chunk_stack: Vec<char> = vec![];
    for char in line.chars() {
        if bracket_close_map
            .keys()
            .find(|open_bracket| char == **open_bracket)
            .is_some()
        {
            chunk_stack.push(char);
        }
        if bracket_close_map
            .values()
            .find(|close_bracket| char == **close_bracket)
            .is_some()
        {
            match chunk_stack.pop() {
                Some(open_bracket) => {
                    if *bracket_close_map.get(&open_bracket).unwrap() != char {
                        return Some(char);
                    }
                }
                None => return Some(char),
            };
        }
    }
    None
}

fn find_completion_string(incomplete_line: &str) -> Vec<char> {
    let bracket_close_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut chunk_stack: Vec<char> = vec![];
    for char in incomplete_line.chars() {
        if bracket_close_map
            .keys()
            .find(|open_bracket| char == **open_bracket)
            .is_some()
        {
            chunk_stack.push(char);
        }
        if bracket_close_map
            .values()
            .find(|close_bracket| char == **close_bracket)
            .is_some()
        {
            match chunk_stack.pop() {
                Some(open_bracket) => {
                    if *bracket_close_map.get(&open_bracket).unwrap() != char {
                        panic!()
                    }
                }
                None => panic!(),
            };
        }
    }
    chunk_stack
        .iter()
        .rev()
        .map(|opening_bracket| bracket_close_map.get(opening_bracket).unwrap().clone())
        .collect::<Vec<char>>()
}

fn line_completion_score(line_completion: Vec<char>) -> usize {
    let bracket_score_map: HashMap<char, usize> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    line_completion
        .iter()
        .fold(0, |acc, x| (acc * 5) + bracket_score_map.get(x).unwrap())
}

fn line_syntax_error_score(corrupted_line: &str) -> usize {
    let syntax_score_map: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    syntax_score_map
        .get(&find_first_illegal_character(corrupted_line).unwrap())
        .unwrap()
        .clone()
}

fn is_corrupted(line: &str) -> bool {
    find_first_illegal_character(line).is_some()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);
    let syntax_error_score: usize = input
        .iter()
        .filter(|line| is_corrupted(line))
        .map(|line| line_syntax_error_score(line))
        .sum();

    let mut line_completion_scores = input
        .iter()
        .filter(|line| !is_corrupted(line))
        .map(|incomplete_line| find_completion_string(incomplete_line))
        .map(|line_completion| line_completion_score(line_completion))
        .collect::<Vec<usize>>();
    line_completion_scores.sort_unstable();
    let middle_score = line_completion_scores[line_completion_scores.len() / 2];

    (syntax_error_score, middle_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 26397);
        assert_eq!(part_2_output, 288957);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
