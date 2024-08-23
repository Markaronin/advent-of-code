use std::str::FromStr;

use advent_of_code_util::{base_aoc, parse::read_parsed_lines, RightOrLeft};
use itertools::Itertools;

/**
 * Note: I modified the input to be a counterclockwise polygon, because otherwise my method of getting edge distance would be invalid.
 * An alternative here would be to calculate whether it is counterclockwise and then change the edge distance calculation but this was easier.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
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
            direction: match &self.color.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction parsed from color"),
            },
            amount: isize::from_str_radix(&self.color[..5], 16).unwrap(),
            color: self.color.clone(),
        }
    }
}

fn turn_direction(prev: Direction, next: Direction) -> RightOrLeft {
    match (prev, next) {
        (Direction::Up, Direction::Left) => RightOrLeft::Left,
        (Direction::Up, Direction::Right) => RightOrLeft::Right,
        (Direction::Down, Direction::Left) => RightOrLeft::Right,
        (Direction::Down, Direction::Right) => RightOrLeft::Left,
        (Direction::Left, Direction::Up) => RightOrLeft::Right,
        (Direction::Left, Direction::Down) => RightOrLeft::Left,
        (Direction::Right, Direction::Up) => RightOrLeft::Left,
        (Direction::Right, Direction::Down) => RightOrLeft::Right,
        _ => panic!("Invalid turn direction"),
    }
}

fn calculate_filled_coordinates(instructions: &[Instruction]) -> usize {
    let mut coords = vec![Coordinate { x: 0, y: 0 }];

    // Count the turn directions surrounding this edge. RR = +1, RL/LR = 0, LL = -1 distance

    for i in 0..instructions.len() {
        let prev_index = (instructions.len() + i - 1) % instructions.len();
        let next_index = (i + 1) % instructions.len();

        let prev_turn_direction = turn_direction(
            instructions[prev_index].direction,
            instructions[i].direction,
        );
        let next_turn_direction = turn_direction(
            instructions[i].direction,
            instructions[next_index].direction,
        );
        let distance_addition = match (prev_turn_direction, next_turn_direction) {
            (RightOrLeft::Right, RightOrLeft::Right) => 1,
            (RightOrLeft::Right, RightOrLeft::Left) => 0,
            (RightOrLeft::Left, RightOrLeft::Right) => 0,
            (RightOrLeft::Left, RightOrLeft::Left) => -1,
        };

        let dist = instructions[i].amount + distance_addition;

        let prev = coords.last().unwrap();
        let next_coord = match instructions[i].direction {
            Direction::Up => Coordinate {
                x: prev.x,
                y: prev.y + dist,
            },
            Direction::Down => Coordinate {
                x: prev.x,
                y: prev.y - dist,
            },
            Direction::Left => Coordinate {
                x: prev.x - dist,
                y: prev.y,
            },
            Direction::Right => Coordinate {
                x: prev.x + dist,
                y: prev.y,
            },
        };
        coords.push(next_coord);
    }

    *coords.last_mut().unwrap() = Coordinate { x: 0, y: 0 };

    let total_area = coords
        .windows(2)
        .map(|w| (w[0].x * w[1].y) - (w[1].x * w[0].y))
        .sum::<isize>()
        .abs()
        / 2;

    total_area.try_into().unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<Instruction> = read_parsed_lines(input_file);

    let result_1 = calculate_filled_coordinates(&input);

    let result_2 =
        calculate_filled_coordinates(&input.iter().map(|i| i.parse_color()).collect_vec());

    (result_1, result_2)
}

base_aoc!(62, 952408144115);
