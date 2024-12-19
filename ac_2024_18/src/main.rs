use std::collections::VecDeque;

use advent_of_code_util::{base_aoc, parse::read_parsed_lines, Coordinate};
use itertools::Itertools;

const SIZE: usize = if cfg!(test) { 7 } else { 71 };
const PART_1_BYTES: usize = if cfg!(test) { 12 } else { 1024 };

fn find_shortest_path(grid: &[[bool; SIZE]; SIZE]) -> Option<Vec<Coordinate>> {
    let mut queue = VecDeque::new();
    let start = Coordinate { x: 0, y: 0 };
    let target = Coordinate {
        x: SIZE - 1,
        y: SIZE - 1,
    };
    let mut prev = [[None; SIZE]; SIZE];
    queue.push_back((None, start));

    while let Some((prev_coord, next)) = queue.pop_front() {
        if next == target {
            let mut path = vec![target];
            let mut p = prev_coord;
            while let Some(c) = p {
                path.push(c);
                p = prev[c.x][c.y];
            }
            path.reverse();
            return Some(path);
        } else {
            prev[next.x][next.y] = prev_coord;
            let surrounding = next
                .get_surrounding_non_diagonal_coordinates(SIZE, SIZE)
                .into_iter()
                .filter(|c| {
                    !grid[c.x][c.y]
                        && prev[c.x][c.y].is_none()
                        && !queue.iter().any(|(_, other)| other == c)
                        && *c != start
                })
                .map(|c| (Some(next), c))
                .collect_vec();
            queue.extend(surrounding);
        }
    }
    None
}

fn get_program_output(input_file: &str) -> (usize, String) {
    let input: Vec<Coordinate> = read_parsed_lines(input_file);

    let answer_1 = {
        let mut grid = [[false; SIZE]; SIZE];
        for byte in &input[..PART_1_BYTES] {
            grid[byte.x][byte.y] = true;
        }

        find_shortest_path(&grid).unwrap().len() - 1
    };
    let answer_2 = 'block: {
        let mut grid = [[false; SIZE]; SIZE];
        let mut current_shortest_path = find_shortest_path(&grid).unwrap();
        for byte in &input {
            grid[byte.x][byte.y] = true;
            if current_shortest_path.contains(byte) {
                match find_shortest_path(&grid) {
                    Some(path) => current_shortest_path = path,
                    None => break 'block format!("{},{}", byte.x, byte.y),
                }
            }
        }
        unreachable!()
    };

    (answer_1, answer_2)
}

base_aoc!(22, "6,1");
