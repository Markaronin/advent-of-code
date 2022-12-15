use std::collections::BTreeSet;

use advent_of_code_util::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}
impl Coordinate {
    pub fn manhatten_distance(&self, other: &Self) -> isize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}
impl Sensor {
    pub fn from_line(line: &str) -> Self {
        let re = Regex::new(r"[xy]=(-?[0-9]+)").unwrap();
        let mut caps = re.captures_iter(line);
        Self {
            position: Coordinate {
                x: caps.next().unwrap()[1].parse().unwrap(),
                y: caps.next().unwrap()[1].parse().unwrap(),
            },
            beacon: Coordinate {
                x: caps.next().unwrap()[1].parse().unwrap(),
                y: caps.next().unwrap()[1].parse().unwrap(),
            },
        }
    }

    // TODO - rename
    pub fn range_intersects(&self, y: isize) -> Vec<Coordinate> {
        let range = self.position.manhatten_distance(&self.beacon);

        let x_radius = range - abs_diff(self.position.y, y);
        let start_x = self.position.x - x_radius;
        let end_x = self.position.x + x_radius;

        (start_x..=end_x).map(|x| Coordinate { x, y }).collect_vec()
    }
}

fn get_program_output(input_file: &str, part_1_y: isize) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| Sensor::from_line(&line))
        .collect_vec();

    let result_1 = input
        .iter()
        .flat_map(|sensor| sensor.range_intersects(part_1_y))
        .collect::<BTreeSet<_>>()
        .difference(&input.iter().map(|sensor| sensor.beacon.clone()).collect())
        .count();

    let result_2 = {
        let only_position = Coordinate { x: 0, y: 0 };
        todo!();
        ((only_position.x * 4000000) + only_position.y) as usize
    };

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path, 10);
        assert_eq!(part_1_output, 26);
        assert_eq!(part_2_output, 56000011);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path, 2000000);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
