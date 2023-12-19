use std::{collections::BTreeSet, str::FromStr};

use advent_of_code_util::{base_aoc, parse::read_parsed_lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}
impl Coordinate {
    pub fn get_points_between_vertices(&self, to: &Coordinate) -> Vec<Coordinate> {
        assert!(self.x == to.x || self.y == to.y);
        match self.x.cmp(&to.x) {
            std::cmp::Ordering::Less => (self.x..=to.x)
                .map(|x| Coordinate { x, y: self.y })
                .collect(),
            std::cmp::Ordering::Equal => {
                if self.y < to.y {
                    (self.y..=to.y)
                        .map(|y| Coordinate { x: self.x, y })
                        .collect()
                } else {
                    (to.y..=self.y)
                        .map(|y| Coordinate { x: self.x, y })
                        .collect()
                }
            }
            std::cmp::Ordering::Greater => (to.x..=self.x)
                .map(|x| Coordinate { x, y: self.y })
                .collect(),
        }
    }
    pub fn get_surrounding_non_diagonal_coordinates(&self) -> Vec<Coordinate> {
        vec![
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Instruction {
    direction: Direction,
    amount: isize,
    color: String,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_dir, raw_amt, raw_color) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let direction = match raw_dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(()),
        };
        let amount = match raw_amt.parse::<isize>() {
            Ok(val) => val,
            Err(_) => return Err(()),
        };
        let color = raw_color[2..raw_color.len() - 1].to_string();

        Ok(Self {
            direction,
            amount,
            color,
        })
    }
}
impl Instruction {
    pub fn parse_color(&self) -> Self {
        Self {
            direction: self.direction,
            amount: isize::from_str_radix(&self.color, 16).unwrap(),
            color: self.color.clone(),
        }
    }
}

fn get_edge_coordinates(dig_plan: &Vec<Instruction>) -> BTreeSet<Coordinate> {
    let mut current_coord = Coordinate { x: 0, y: 0 };
    let mut vertical_edge_coordinates = BTreeSet::new();
    for instruction in dig_plan {
        let next_coordinate = match instruction.direction {
            Direction::Up => Coordinate {
                x: current_coord.x,
                y: current_coord.y - instruction.amount,
            },
            Direction::Down => Coordinate {
                x: current_coord.x,
                y: current_coord.y + instruction.amount,
            },
            Direction::Left => Coordinate {
                x: current_coord.x - instruction.amount,
                y: current_coord.y,
            },
            Direction::Right => Coordinate {
                x: current_coord.x + instruction.amount,
                y: current_coord.y,
            },
        };
        let edge_coords = current_coord.get_points_between_vertices(&next_coordinate);
        vertical_edge_coordinates.extend(edge_coords);
        current_coord = next_coordinate;
    }
    vertical_edge_coordinates
}

fn get_filled_coordinates(edge_coordinates: &BTreeSet<Coordinate>) -> BTreeSet<Coordinate> {
    let mut filled_coords = BTreeSet::new();
    filled_coords.append(&mut edge_coordinates.clone());

    let mut queue = BTreeSet::new();

    let starting_coordinate = if cfg!(test) {
        Coordinate { x: 1, y: 1 }
    } else {
        Coordinate { x: 1, y: -1 }
    };

    queue.insert(starting_coordinate);

    while let Some(next) = queue.pop_first() {
        for surrounding in next.get_surrounding_non_diagonal_coordinates() {
            if !queue.contains(&surrounding) && !filled_coords.contains(&surrounding) {
                queue.insert(surrounding);
            }
        }

        filled_coords.insert(next);
    }

    filled_coords
}

fn calculate_filled_coordinates(instructions: &Vec<Instruction>) -> usize {
    // We can just look at "squares" caused by the fact that we're in a loop
    // NEW IDEA: Take segments of Up-Over-Down (or their equivalents), chop off the square, and turn it into just over
    unimplemented!()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<Instruction> = read_parsed_lines(input_file);

    let result_1 = {
        let edges = get_edge_coordinates(&input);
        let filled_area = get_filled_coordinates(&edges);
        filled_area.len()
    };

    let result_2 =
        calculate_filled_coordinates(&input.iter().map(|i| i.parse_color()).collect_vec());

    (result_1, result_2)
}

base_aoc!(62, 952408144115);
