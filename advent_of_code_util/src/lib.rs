use itertools::Itertools;
use std::{collections::BTreeSet, str::FromStr};

pub mod matrix;
pub mod parse;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RightOrLeft {
    Right,
    Left,
}

pub fn abs_diff<T: Ord + std::ops::Sub<Output = T> + Copy>(slf: T, other: T) -> T {
    std::cmp::max(slf, other) - std::cmp::min(slf, other)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap();
        Ok(Coordinate { x, y })
    }
}
impl Coordinate {
    pub fn space_in_direction(
        &self,
        direction: Direction,
        max_height: usize,
        max_width: usize,
    ) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < max_height - 1 {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < max_width - 1 {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    /**
    assumes that from and to are either on a horizontal or vertical line
    */
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

    pub fn non_diagonal_distance(&self, other: &Coordinate) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }

    pub fn is_within_bounds(&self, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
        self.x >= min_x && self.x <= max_x && self.y >= min_y && self.y <= max_y
    }

    pub fn get_surrounding_non_diagonal_coordinates(
        &self,
        max_width: usize,
        max_height: usize,
    ) -> Vec<Coordinate> {
        let mut surrounding_coordinates = vec![];
        if self.x > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < max_width - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < max_height - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
        }
        surrounding_coordinates
    }
    pub fn get_surrounding_coordinates(
        &self,
        max_width: usize,
        max_height: usize,
    ) -> Vec<Coordinate> {
        let mut surrounding_coordinates = vec![];
        if self.x > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
            if self.y > 0 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
        }
        if self.y > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
            if self.x < max_width - 1 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x + 1,
                    y: self.y - 1,
                });
            }
        }
        if self.x < max_width - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
            if self.y < max_height - 1 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x + 1,
                    y: self.y + 1,
                });
            }
        }
        if self.y < max_height - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
            if self.x > 0 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x - 1,
                    y: self.y + 1,
                });
            }
        }
        surrounding_coordinates
    }
}

pub fn remove_first_and_last(string: &str) -> String {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}

pub fn intersect_vectors<T: std::cmp::Ord>(vecs: Vec<Vec<T>>) -> Vec<T> {
    let mut vec_iter = vecs.into_iter();
    let mut remaining = BTreeSet::from_iter(vec_iter.next().unwrap());

    for vec in vec_iter {
        let vec_set = BTreeSet::from_iter(vec.into_iter());
        remaining.retain(|item| vec_set.contains(item));
    }

    remaining.into_iter().collect_vec()
}

#[macro_export]
macro_rules! base_aoc {
    ( $part_1_answer:literal, $part_2_answer:literal ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn main() {
                let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
                let (part_1_output, part_2_output) = get_program_output(&file_path);
                assert_eq!(part_1_output, $part_1_answer);
                assert_eq!(part_2_output, $part_2_answer);
            }
        }

        fn main() {
            let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
            let (part_1_output, part_2_output) = get_program_output(&file_path);
            println!("Part 1 output: {}", part_1_output);
            println!("Part 2 output: {}", part_2_output);
        }
    };
}

#[macro_export]
macro_rules! base_aoc_ignore_tests {
    ( $part_1_answer:literal, $part_2_answer:literal ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[ignore]
            #[test]
            fn main() {
                let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
                let (part_1_output, part_2_output) = get_program_output(&file_path);
                assert_eq!(part_1_output, $part_1_answer);
                assert_eq!(part_2_output, $part_2_answer);
            }
        }

        fn main() {
            let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
            let (part_1_output, part_2_output) = get_program_output(&file_path);
            println!("Part 1 output: {}", part_1_output);
            println!("Part 2 output: {}", part_2_output);
        }
    };
}
