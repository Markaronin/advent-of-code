use advent_of_code_util::{base_aoc, parse::read_lines, Coordinate};
use itertools::Itertools;
use regex::Regex;

fn is_part_number(grid: &Vec<String>, line_number: usize, start: usize, end: usize) -> bool {
    let start_x = start.max(1) - 1;
    let end_x = (end + 1).min(grid.len() - 1);
    let start_y = line_number.max(1) - 1;
    let end_y = (line_number + 1).min(grid[0].len() - 1);

    for y in start_y..=end_y {
        let chars = grid[y].chars().collect_vec();
        for x in start_x..=end_x {
            if chars[x] != '.' && !chars[x].is_numeric() {
                return true;
            }
        }
    }
    false
}

struct NumberAndPosition {
    number: usize,
    line_number: usize,
    start_index: usize,
    end_index: usize,
}
impl NumberAndPosition {
    pub fn contains_coordinate(&self, coord: &Coordinate) -> bool {
        coord.y == self.line_number && (self.start_index <= coord.x && coord.x <= self.end_index)
    }
}

fn get_all_numbers_and_positions(grid: &Vec<String>) -> Vec<NumberAndPosition> {
    let mut result = Vec::new();
    for (line_number, line) in grid.iter().enumerate() {
        // Create a regular expression pattern to match numbers with one or more digits
        let re = Regex::new(r"\d+").unwrap();

        // Find all matches in the input string
        for capture in re.captures_iter(line) {
            let number = capture.get(0).unwrap().as_str().parse::<usize>().unwrap();
            let start = capture.get(0).unwrap().start();
            let end = capture.get(0).unwrap().end() - 1;
            result.push(NumberAndPosition {
                number,
                line_number,
                start_index: start,
                end_index: end,
            });
        }
    }
    result
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let all_numbers_and_positions = get_all_numbers_and_positions(&input);

    let mut a = 0;
    for num in &all_numbers_and_positions {
        if is_part_number(&input, num.line_number, num.start_index, num.end_index) {
            a += num.number;
        }
    }

    let mut b = 0;
    let all_gear_coordinates = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '*' {
                    Some(Coordinate { x, y })
                } else {
                    None
                }
            })
        })
        .collect_vec();

    for gear_coordinate in all_gear_coordinates {
        let mut surrounding_scores = Vec::new();
        for num in &all_numbers_and_positions {
            if gear_coordinate
                .get_surrounding_coordinates(input[0].len(), input.len())
                .iter()
                .any(|surrounding_coordinate| num.contains_coordinate(surrounding_coordinate))
            {
                surrounding_scores.push(num.number);
            }
        }
        if surrounding_scores.len() == 2 {
            b += surrounding_scores.into_iter().product::<usize>();
        }
    }

    (a, b)
}

base_aoc!(4361, 467835);
