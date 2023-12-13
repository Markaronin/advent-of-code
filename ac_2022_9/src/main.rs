use std::{cmp::Ordering, collections::BTreeSet};

use advent_of_code_util::{abs_diff, base_aoc, parse::read_lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}
impl Coordinate {
    pub fn is_touching(&self, other: &Coordinate) -> bool {
        abs_diff(self.x, other.x) <= 1 && abs_diff(self.y, other.y) <= 1
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unrecognized direction {c}"),
        }
    }
    pub fn apply(&self, c: &Coordinate) -> Coordinate {
        match self {
            Self::Up => Coordinate { x: c.x, y: c.y + 1 },
            Self::Down => Coordinate { x: c.x, y: c.y - 1 },
            Self::Left => Coordinate { x: c.x - 1, y: c.y },
            Self::Right => Coordinate { x: c.x + 1, y: c.y },
        }
    }
}

fn move_tail(head: &Coordinate, tail: &mut Coordinate) {
    if !head.is_touching(&tail) {
        match tail.x.cmp(&head.x) {
            Ordering::Less => tail.x += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.x -= 1,
        }
        match tail.y.cmp(&head.y) {
            Ordering::Less => tail.y += 1,
            Ordering::Equal => (),
            Ordering::Greater => tail.y -= 1,
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            let mut line_split = line.split_ascii_whitespace();
            let direction =
                Direction::from_char(line_split.next().unwrap().chars().next().unwrap());
            let amt = line_split.next().unwrap().parse::<usize>().unwrap();
            (direction, amt)
        })
        .collect_vec();

    let mut head_position = Coordinate { x: 0, y: 0 };
    let mut tail_positions = [Coordinate { x: 0, y: 0 }; 9];
    let mut first_tail_positions_visited: BTreeSet<Coordinate> = BTreeSet::new();
    let mut last_tail_positions_visited: BTreeSet<Coordinate> = BTreeSet::new();
    first_tail_positions_visited.insert(tail_positions[0].clone());
    last_tail_positions_visited.insert(tail_positions[8].clone());

    for (direction, amount) in input {
        for _ in 0..amount {
            head_position = direction.apply(&head_position);
            move_tail(&head_position, &mut tail_positions[0]);
            for i in 1..=8 {
                let split_slice = tail_positions.split_at_mut(i);
                move_tail(&split_slice.0[i - 1], &mut split_slice.1[0]);
            }
            first_tail_positions_visited.insert(tail_positions[0].clone());
            last_tail_positions_visited.insert(tail_positions[8].clone());
        }
    }

    let result_1 = first_tail_positions_visited.len();
    let result_2 = last_tail_positions_visited.len();

    (result_1, result_2)
}

base_aoc!(13, 1);
