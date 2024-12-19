use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate, Direction};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    facing: Direction,
}
impl Node {
    pub fn surrounding_nodes_with_costs(&self, grid: &[Vec<char>]) -> Vec<(Self, usize)> {
        match self.facing {
            Direction::Up => vec![
                (
                    Self {
                        x: self.x,
                        y: self.y - 1,
                        facing: Direction::Up,
                    },
                    1,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Right,
                    },
                    1000,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Left,
                    },
                    1000,
                ),
            ],
            Direction::Down => vec![
                (
                    Self {
                        x: self.x,
                        y: self.y + 1,
                        facing: Direction::Down,
                    },
                    1,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Right,
                    },
                    1000,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Left,
                    },
                    1000,
                ),
            ],
            Direction::Right => vec![
                (
                    Self {
                        x: self.x + 1,
                        y: self.y,
                        facing: Direction::Right,
                    },
                    1,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Up,
                    },
                    1000,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Down,
                    },
                    1000,
                ),
            ],
            Direction::Left => vec![
                (
                    Self {
                        x: self.x - 1,
                        y: self.y,
                        facing: Direction::Left,
                    },
                    1,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Up,
                    },
                    1000,
                ),
                (
                    Self {
                        x: self.x,
                        y: self.y,
                        facing: Direction::Down,
                    },
                    1000,
                ),
            ],
        }
        .into_iter()
        .filter(|(n, _)| grid[n.y][n.x] != '#')
        .collect_vec()
    }
}
impl PartialEq<Coordinate> for Node {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl From<Node> for Coordinate {
    fn from(value: Node) -> Self {
        Coordinate {
            x: value.x,
            y: value.y,
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let end_coordinate = Coordinate {
        x: input[0].len() - 2,
        y: 1,
    };

    let mut distances = BTreeMap::new();
    let mut heap = BinaryHeap::new();
    let mut nodes_on_path: BTreeMap<Node, BTreeSet<Coordinate>> = BTreeMap::new();

    heap.push((
        Reverse(0),
        Node {
            x: 1,
            y: input.len() - 2,
            facing: Direction::Right,
        },
    ));
    nodes_on_path.insert(
        Node {
            x: 1,
            y: input.len() - 2,
            facing: Direction::Right,
        },
        BTreeSet::new(),
    );

    while let Some((Reverse(cost), position)) = heap.pop() {
        if cost > *distances.entry(position).or_insert(usize::MAX) {
            continue;
        }
        for (next_pos, next_cost) in position.surrounding_nodes_with_costs(&input) {
            let next_cost = cost + next_cost;
            // If so, add it to the frontier and continue
            match next_cost.cmp(distances.entry(next_pos).or_insert(usize::MAX)) {
                std::cmp::Ordering::Less => {
                    // Relaxation, we have now found a better way
                    heap.push((Reverse(next_cost), next_pos));
                    distances.insert(next_pos, next_cost);

                    let mut next_nodes_on_path = nodes_on_path.get(&position).unwrap().clone();
                    next_nodes_on_path.insert(next_pos.into());
                    nodes_on_path.insert(next_pos, next_nodes_on_path);
                }
                std::cmp::Ordering::Equal => {
                    let other_nodes_on_path = nodes_on_path.get(&position).unwrap().clone();
                    nodes_on_path
                        .get_mut(&next_pos)
                        .unwrap()
                        .extend(other_nodes_on_path);
                }
                std::cmp::Ordering::Greater => {}
            };
            if next_cost < *distances.entry(next_pos).or_insert(usize::MAX) {
                heap.push((Reverse(next_cost), next_pos));
                // Relaxation, we have now found a better way
                distances.insert(next_pos, next_cost);

                let mut next_nodes_on_path = nodes_on_path.get(&position).unwrap().clone();
                next_nodes_on_path.insert(next_pos.into());
                nodes_on_path.insert(next_pos, next_nodes_on_path);
            }
        }
    }

    let answer_1 = *[
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .flat_map(|dir| {
        distances.get(&Node {
            x: end_coordinate.x,
            y: end_coordinate.y,
            facing: dir,
        })
    })
    .min()
    .unwrap();

    let answer_2 = [
        Direction::Down,
        Direction::Up,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .map(|dir| Node {
        x: end_coordinate.x,
        y: end_coordinate.y,
        facing: dir,
    })
    .filter(|node| distances.get(node) == Some(&answer_1))
    .flat_map(|node| nodes_on_path.get(&node))
    .flatten()
    .collect::<BTreeSet<_>>()
    .len();

    (answer_1, answer_2)
}

base_aoc!(11048, 64);
