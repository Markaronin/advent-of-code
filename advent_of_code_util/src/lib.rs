use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn read_lines_of_chars<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect_vec())
        .collect()
}

pub fn read_blocks<P>(filename: P) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut blocks = vec![];
    let mut latest_block = vec![];
    for line in io::BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            blocks.push(latest_block);
            latest_block = vec![];
        } else {
            latest_block.push(line);
        }
    }
    if !latest_block.is_empty() {
        blocks.push(latest_block);
    }
    blocks
}

pub fn split_block_on_whitespace(block: Vec<String>) -> Vec<String> {
    block
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|split_line| split_line.to_string())
        .collect::<Vec<String>>()
}

pub fn abs_diff(slf: usize, other: usize) -> usize {
    std::cmp::max(slf, other) - std::cmp::min(slf, other)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
impl Coordinate {
    pub fn from_str(string: &str) -> Self {
        let (x, y) = string
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap();
        Coordinate { x, y }
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn split_block_on_whitespace_test() {
        assert_eq!(
            split_block_on_whitespace(vec![
                "pid:161cm eyr:2025 hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024 byr:2012".to_string()
            ]),
            vec![
                "pid:161cm".to_string(),
                "eyr:2025".to_string(),
                "hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024".to_string(),
                "byr:2012".to_string()
            ]
        );
    }
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
