use std::collections::{BTreeSet, VecDeque};

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calculate_light_path(
    grid: &[Vec<char>],
    current_pos: Coordinate,
    current_direction: Direction,
) -> Vec<(Coordinate, Direction)> {
    let current_space = grid[current_pos.y][current_pos.x];
    match current_space {
        '.' => next_pos_straight_line(grid, current_pos, current_direction)
            .map(|next| vec![next])
            .unwrap_or(vec![]),
        '/' => {
            let new_direction = match current_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            next_pos_straight_line(grid, current_pos, new_direction)
                .map(|next| vec![next])
                .unwrap_or(vec![])
        }
        '\\' => {
            let new_direction = match current_direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            next_pos_straight_line(grid, current_pos, new_direction)
                .map(|next| vec![next])
                .unwrap_or(vec![])
        }
        '|' => match current_direction {
            Direction::Up | Direction::Down => {
                next_pos_straight_line(grid, current_pos, current_direction)
                    .map(|next| vec![next])
                    .unwrap_or(vec![])
            }
            Direction::Left | Direction::Right => [
                next_pos_straight_line(grid, current_pos, Direction::Up),
                next_pos_straight_line(grid, current_pos, Direction::Down),
            ]
            .into_iter()
            .flatten()
            .collect_vec(),
        },
        '-' => match current_direction {
            Direction::Left | Direction::Right => {
                next_pos_straight_line(grid, current_pos, current_direction)
                    .map(|next| vec![next])
                    .unwrap_or(vec![])
            }
            Direction::Up | Direction::Down => [
                next_pos_straight_line(grid, current_pos, Direction::Left),
                next_pos_straight_line(grid, current_pos, Direction::Right),
            ]
            .into_iter()
            .flatten()
            .collect_vec(),
        },
        _ => panic!("Invalid space type"),
    }
}

fn next_pos_straight_line(
    grid: &[Vec<char>],
    pos: Coordinate,
    direction: Direction,
) -> Option<(Coordinate, Direction)> {
    match direction {
        Direction::Up => {
            if pos.y > 0 {
                Some((
                    Coordinate {
                        x: pos.x,
                        y: pos.y - 1,
                    },
                    direction,
                ))
            } else {
                None
            }
        }
        Direction::Down => {
            if pos.y < grid.len() - 1 {
                Some((
                    Coordinate {
                        x: pos.x,
                        y: pos.y + 1,
                    },
                    direction,
                ))
            } else {
                None
            }
        }
        Direction::Left => {
            if pos.x > 0 {
                Some((
                    Coordinate {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                    direction,
                ))
            } else {
                None
            }
        }
        Direction::Right => {
            if pos.x < grid[0].len() - 1 {
                Some((
                    Coordinate {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    direction,
                ))
            } else {
                None
            }
        }
    }
}

fn calculate_energized_tiles(grid: &[Vec<char>], start: (Coordinate, Direction)) -> usize {
    let mut cache: BTreeSet<(Coordinate, Direction)> = BTreeSet::new();
    let mut queue: VecDeque<(Coordinate, Direction)> = VecDeque::new();
    queue.push_back(start);
    cache.insert(start);

    while let Some((position, direction)) = queue.pop_front() {
        let mut next_tiles = calculate_light_path(grid, position, direction);

        next_tiles.retain(|n| !cache.contains(n));

        cache.extend(next_tiles.clone());
        queue.extend(next_tiles);
    }

    cache.into_iter().map(|(pos, _)| pos).unique().count()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let result_1 = calculate_energized_tiles(&input, (Coordinate { x: 0, y: 0 }, Direction::Right));

    let result_2 = {
        let mut possible_starts = vec![];
        let max_y = input.len() - 1;
        let max_x = input[0].len() - 1;
        for y in 0..=max_y {
            possible_starts.push((Coordinate { x: 0, y }, Direction::Right));
            possible_starts.push((Coordinate { x: max_x, y }, Direction::Left));
        }
        for x in 0..=max_x {
            possible_starts.push((Coordinate { x, y: 0 }, Direction::Down));
            possible_starts.push((Coordinate { x, y: max_y }, Direction::Up));
        }

        possible_starts
            .into_iter()
            .map(|start| calculate_energized_tiles(&input, start))
            .max()
            .unwrap()
    };

    (result_1, result_2)
}

base_aoc!(46, 51);
