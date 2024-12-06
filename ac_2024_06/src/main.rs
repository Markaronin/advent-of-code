use std::collections::BTreeSet;

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate, Direction};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Blocked,
}

fn number_of_unique_spaces_visited_if_exits(
    spaces: &[Vec<Space>],
    mut guard_position: Coordinate,
    mut guard_direction: Direction,
) -> Option<usize> {
    let mut visited = BTreeSet::new();
    visited.insert((guard_position, guard_direction));

    while let Some(next_space) =
        guard_position.space_in_direction(guard_direction, spaces.len(), spaces[0].len())
    {
        match spaces[next_space.y][next_space.x] {
            Space::Empty => {
                guard_position = next_space;
            }
            Space::Blocked => {
                guard_direction = match guard_direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                }
            }
        }
        if visited.contains(&(guard_position, guard_direction)) {
            return None;
        } else {
            visited.insert((guard_position, guard_direction));
        }
    }

    Some(visited.into_iter().map(|(pos, _)| pos).unique().count())
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let mut spaces: Vec<Vec<Space>> = Vec::new();
    let mut guard_position = None;
    for (y, row) in input.into_iter().enumerate() {
        spaces.push(Vec::new());
        for (x, col) in row.into_iter().enumerate() {
            spaces[y].push(match col {
                '#' => Space::Blocked,
                '.' => Space::Empty,
                '^' => {
                    guard_position = Some(Coordinate { x, y });
                    Space::Empty
                }
                _ => unreachable!(),
            })
        }
    }
    let guard_position = guard_position.unwrap();
    let guard_direction = Direction::Up;

    let answer_1 =
        number_of_unique_spaces_visited_if_exits(&spaces, guard_position, guard_direction).unwrap();

    let mut answer_2 = 0;
    for y in 0..spaces.len() {
        for x in 0..spaces[0].len() {
            if spaces[y][x] != Space::Blocked && guard_position != (Coordinate { x, y }) {
                spaces[y][x] = Space::Blocked;

                if number_of_unique_spaces_visited_if_exits(
                    &spaces,
                    guard_position,
                    guard_direction,
                )
                .is_none()
                {
                    answer_2 += 1;
                }

                spaces[y][x] = Space::Empty;
            }
        }
    }

    (answer_1, answer_2)
}

base_aoc!(41, 6);
