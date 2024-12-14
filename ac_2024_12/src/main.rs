use std::collections::BTreeSet;

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;

#[derive(Debug)]
struct Area {
    #[allow(unused)]
    id: char,
    coords: BTreeSet<Coordinate>,
}
impl Area {
    pub fn all_areas_from_grid(input: &[Vec<char>]) -> Vec<Self> {
        let mut areas = Vec::new();

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                let coord = Coordinate { x, y };
                if !areas.iter().any(|area: &Area| area.coords.contains(&coord)) {
                    let mut coords = BTreeSet::new();
                    let mut queue = vec![coord];
                    while let Some(next_coord) = queue.pop() {
                        coords.insert(next_coord);
                        let surrounding_coords = next_coord
                            .get_surrounding_non_diagonal_coordinates(input[0].len(), input.len())
                            .into_iter()
                            .filter(|c| !queue.contains(c))
                            .filter(|c| !coords.contains(c))
                            .filter(|c| input[c.y][c.x] == input[coord.y][coord.x])
                            .collect_vec();
                        queue.extend(surrounding_coords);
                    }
                    areas.push(Area {
                        id: input[coord.y][coord.x],
                        coords,
                    });
                }
            }
        }

        areas
    }

    pub fn area(&self) -> usize {
        self.coords.len()
    }

    pub fn perimeter(&self, max_height: usize, max_width: usize) -> usize {
        self.coords
            .iter()
            .map(|c| {
                4 - c
                    .get_surrounding_non_diagonal_coordinates(max_width, max_height)
                    .into_iter()
                    .filter(|sc| self.coords.contains(sc))
                    .count()
            })
            .sum()
    }

    pub fn num_edges(&self, max_height: usize, max_width: usize) -> usize {
        // For each cardinal direction,
        // Draw a line straight up->down or left -> right, along the coordinates that this area has.
        // Count 1 for each uninterrupted set of coordinates with that edge
        let min_x = self.coords.iter().map(|c| c.x).min().unwrap();
        let max_x = self.coords.iter().map(|c| c.x).max().unwrap();
        let min_y = self.coords.iter().map(|c| c.y).min().unwrap();
        let max_y = self.coords.iter().map(|c| c.y).max().unwrap();

        let mut edges = 0;

        // Check up->down (right and left edges)
        for x in min_x..=max_x {
            let mut left_edge = false;
            let mut right_edge = false;
            for y in min_y..=max_y {
                let coord = Coordinate { x, y };
                if self.coords.contains(&coord) {
                    let left_side_covered = x != 0
                        && self.coords.contains(&Coordinate {
                            x: coord.x - 1,
                            y: coord.y,
                        });
                    let right_side_covered = x != max_width - 1
                        && self.coords.contains(&Coordinate {
                            x: coord.x + 1,
                            y: coord.y,
                        });

                    if left_side_covered {
                        left_edge = false;
                    } else if !left_edge {
                        left_edge = true;
                        edges += 1;
                    }
                    if right_side_covered {
                        right_edge = false;
                    } else if !right_edge {
                        right_edge = true;
                        edges += 1;
                    }
                } else {
                    left_edge = false;
                    right_edge = false;
                }
            }
        }

        // Check left->right (top and bottom edges)
        for y in min_y..=max_y {
            let mut top_edge = false;
            let mut bottom_edge = false;
            for x in min_x..=max_x {
                let coord = Coordinate { x, y };
                if self.coords.contains(&coord) {
                    let top_side_covered = y != 0
                        && self.coords.contains(&Coordinate {
                            x: coord.x,
                            y: coord.y - 1,
                        });
                    let bottom_side_covered = y != max_height - 1
                        && self.coords.contains(&Coordinate {
                            x: coord.x,
                            y: coord.y + 1,
                        });

                    if top_side_covered {
                        top_edge = false;
                    } else if !top_edge {
                        top_edge = true;
                        edges += 1;
                    }
                    if bottom_side_covered {
                        bottom_edge = false;
                    } else if !bottom_edge {
                        bottom_edge = true;
                        edges += 1;
                    }
                } else {
                    top_edge = false;
                    bottom_edge = false;
                }
            }
        }

        edges
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);
    let areas = Area::all_areas_from_grid(&input);

    let answer_1 = areas
        .iter()
        .map(|area| area.area() * area.perimeter(input.len(), input[0].len()))
        .sum();

    let answer_2 = areas
        .iter()
        .map(|area| area.area() * area.num_edges(input.len(), input[0].len()))
        .sum();

    (answer_1, answer_2)
}

base_aoc!(1930, 1206);
