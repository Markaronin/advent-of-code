use advent_of_code_util::{base_aoc, parse::read_lines_of_chars};
use itertools::Itertools;

fn count_occurances_of_word(puzzle: &[Vec<char>], word: &str) -> usize {
    let word_chars = word.chars().collect_vec();
    let mut occurances = 0;
    // Up + Down
    for y in 0..puzzle.len() + 1 - word.len() {
        for x in 0..puzzle.len() {
            if (0..word.len()).all(|i| puzzle[y + i][x] == word_chars[i]) {
                occurances += 1;
            }
            if (0..word.len()).all(|i| puzzle[y + word.len() - 1 - i][x] == word_chars[i]) {
                occurances += 1;
            }
        }
    }
    // Left + Right
    for y in 0..puzzle.len() {
        for x in 0..puzzle.len() + 1 - word.len() {
            if (0..word.len()).all(|i| puzzle[y][x + i] == word_chars[i]) {
                occurances += 1;
            }
            if (0..word.len()).all(|i| puzzle[y][x + word.len() - 1 - i] == word_chars[i]) {
                occurances += 1;
            }
        }
    }
    // Diag DR + UL + UR + DL
    for y in 0..puzzle.len() + 1 - word.len() {
        for x in 0..puzzle.len() + 1 - word.len() {
            if (0..word.len()).all(|i| puzzle[y + i][x + i] == word_chars[i]) {
                occurances += 1;
            }
            if (0..word.len()).all(|i| puzzle[y + i][x + word.len() - 1 - i] == word_chars[i]) {
                occurances += 1;
            }
            if (0..word.len()).all(|i| puzzle[y + word.len() - 1 - i][x + i] == word_chars[i]) {
                occurances += 1;
            }
            if (0..word.len())
                .all(|i| puzzle[y + word.len() - 1 - i][x + word.len() - 1 - i] == word_chars[i])
            {
                occurances += 1;
            }
        }
    }
    occurances
}

fn count_x_mas(puzzle: &[Vec<char>]) -> usize {
    let mut occurances = 0;
    for y in 0..puzzle.len() - 2 {
        for x in 0..puzzle.len() - 2 {
            let dr = (0..3).map(|i| puzzle[y + i][x + i]).collect::<String>();
            let ul = (0..3)
                .map(|i| puzzle[y + 2 - i][x + 2 - i])
                .collect::<String>();

            let ur = (0..3).map(|i| puzzle[y + i][x + 2 - i]).collect::<String>();
            let dl = (0..3).map(|i| puzzle[y + 2 - i][x + i]).collect::<String>();

            if (dr == "MAS" || ul == "MAS") && (ur == "MAS" || dl == "MAS") {
                occurances += 1;
            }
        }
    }
    occurances
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let answer_1 = count_occurances_of_word(&input, "XMAS");
    let answer_2 = count_x_mas(&input);

    (answer_1, answer_2)
}

base_aoc!(18, 9);
