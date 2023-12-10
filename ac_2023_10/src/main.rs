use std::collections::{BTreeSet, VecDeque};

use advent_of_code_util::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    UpAndLeft,
    UpAndRight,
    UpAndDown,
    LeftAndRight,
    LeftAndDown,
    RightAndDown,
    Empty,
    Animal,
}
impl Space {
    pub fn from_char(c: char) -> Self {
        match c {
            'J' => Self::UpAndLeft,
            'L' => Self::UpAndRight,
            '|' => Self::UpAndDown,
            '-' => Self::LeftAndRight,
            '7' => Self::LeftAndDown,
            'F' => Self::RightAndDown,
            '.' => Self::Empty,
            'S' => Self::Animal,
            _ => panic!("Invalid space {c}"),
        }
    }
    pub fn connected_pipes(coord: Coordinate, grid: &Grid) -> Vec<Coordinate> {
        match grid[coord.y][coord.x] {
            Space::UpAndLeft => vec![
                Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                },
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                },
            ],
            Space::UpAndRight => vec![
                Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                },
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                },
            ],
            Space::UpAndDown => vec![
                Coordinate {
                    x: coord.x,
                    y: coord.y - 1,
                },
                Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                },
            ],
            Space::LeftAndRight => vec![
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                },
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                },
            ],
            Space::LeftAndDown => vec![
                Coordinate {
                    x: coord.x - 1,
                    y: coord.y,
                },
                Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                },
            ],
            Space::RightAndDown => vec![
                Coordinate {
                    x: coord.x + 1,
                    y: coord.y,
                },
                Coordinate {
                    x: coord.x,
                    y: coord.y + 1,
                },
            ],
            Space::Empty => vec![],
            Space::Animal => coord
                .get_surrounding_non_diagonal_coordinates(grid[0].len(), grid.len())
                .into_iter()
                .filter(|s| Space::connected_pipes(*s, grid).contains(&coord))
                .collect_vec(),
        }
    }
}

type Grid = Vec<Vec<Space>>;

fn is_enclosed(mut ray: VecDeque<Space>) -> bool {
    ray = ray
        .into_iter()
        .filter(|s| *s != Space::Empty && *s != Space::LeftAndRight)
        .collect();
    let mut walls = 0;
    while ray.len() > 0 {
        // dbg!(&ray);
        if ray[0] == Space::UpAndDown {
            ray.pop_front().unwrap();
            walls += 1;
        } else if ray[0] == Space::RightAndDown && ray[1] == Space::UpAndLeft {
            ray.pop_front().unwrap();
            ray.pop_front().unwrap();
            walls += 1;
        } else if ray[0] == Space::UpAndRight && ray[1] == Space::LeftAndDown {
            ray.pop_front().unwrap();
            ray.pop_front().unwrap();
            walls += 1;
        } else if ray[0] == Space::UpAndRight && ray[1] == Space::UpAndLeft {
            ray.pop_front().unwrap();
            ray.pop_front().unwrap();
        } else if ray[0] == Space::RightAndDown && ray[1] == Space::LeftAndDown {
            ray.pop_front().unwrap();
            ray.pop_front().unwrap();
        }
    }
    walls % 2 != 0
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file)
        .into_iter()
        .map(|line| line.into_iter().map(|c| Space::from_char(c)).collect_vec())
        .collect_vec();

    let starting_position = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, space)| {
                if *space == Space::Animal {
                    Some(Coordinate { x, y })
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap();

    let mut seen_pipes: BTreeSet<Coordinate> = BTreeSet::new();
    seen_pipes.insert(starting_position);
    let mut current_pipes = vec![Space::connected_pipes(starting_position, &input)[0]];
    let mut distance = 1;

    while !current_pipes.is_empty() {
        seen_pipes.extend(current_pipes.iter());

        current_pipes = current_pipes
            .iter()
            .flat_map(|pipe| Space::connected_pipes(*pipe, &input))
            .filter(|pipe| !seen_pipes.contains(pipe))
            .collect_vec();

        distance += 1;
    }

    let mut part_2_grid = input.clone();
    // Step 1: Delete all non-path pipes
    for y in 0..part_2_grid.len() {
        for x in 0..part_2_grid[0].len() {
            if !seen_pipes.contains(&Coordinate { x, y }) {
                part_2_grid[y][x] = Space::Empty;
            }
        }
    }
    // Step 1.5: Replace "S" with pipe
    if part_2_grid.len() == 10 {
        part_2_grid[starting_position.y][starting_position.x] = Space::LeftAndDown;
    } else {
        part_2_grid[starting_position.y][starting_position.x] = Space::UpAndLeft;
    }

    // Step 2: for every non-fence space, cast a ray to the right (i.e. get all spaces between this space and the right edge)
    let mut num_enclosed_spaces = 0;
    for y in 0..part_2_grid.len() {
        for x in 0..part_2_grid[0].len() {
            if part_2_grid[y][x] == Space::Empty {
                let ray = part_2_grid[y][x..]
                    .iter()
                    .map(|s| *s)
                    .collect::<VecDeque<_>>();
                if is_enclosed(ray) {
                    num_enclosed_spaces += 1;
                }
            }
        }
    }

    (distance / 2, num_enclosed_spaces)
}

base_aoc!(80, 10);
