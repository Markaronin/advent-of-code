use std::collections::BTreeMap;

use advent_of_code_util::{base_aoc, icoordinate::ICoordinate, parse::read_lines_of_chars};
use itertools::Itertools;

struct State {
    max_height: isize,
    max_width: isize,
    nodes: BTreeMap<char, Vec<ICoordinate>>,
}
impl State {
    pub fn all_antinodes(
        &self,
        only_first: bool,
        include_nodes: bool,
    ) -> BTreeMap<char, Vec<ICoordinate>> {
        let mut antinodes = BTreeMap::new();

        for (k, v) in &self.nodes {
            let mut antinodes_for_char = Vec::new();

            if include_nodes {
                antinodes_for_char.extend(v);
            }
            for (a, b) in v.iter().tuple_combinations() {
                if only_first {
                    antinodes_for_char.extend(self.get_positive_antinodes(*a, *b).first());
                    antinodes_for_char.extend(self.get_negative_antinodes(*a, *b).first());
                } else {
                    antinodes_for_char.extend(self.get_positive_antinodes(*a, *b));
                    antinodes_for_char.extend(self.get_negative_antinodes(*a, *b));
                }
            }

            antinodes.insert(*k, antinodes_for_char);
        }

        antinodes
    }
    fn get_positive_antinodes(&self, a: ICoordinate, b: ICoordinate) -> Vec<ICoordinate> {
        let diff = b - a;
        let mut current = b + diff;
        let mut antinodes = Vec::new();
        while current.x >= 0
            && current.x < self.max_width
            && current.y >= 0
            && current.y < self.max_height
        {
            antinodes.push(current);
            current = current + diff;
        }
        antinodes
    }
    fn get_negative_antinodes(&self, a: ICoordinate, b: ICoordinate) -> Vec<ICoordinate> {
        let diff = b - a;
        let mut current = a - diff;
        let mut antinodes = Vec::new();
        while current.x >= 0
            && current.x < self.max_width
            && current.y >= 0
            && current.y < self.max_height
        {
            antinodes.push(current);
            current = current - diff;
        }
        antinodes
    }
}
impl From<Vec<Vec<char>>> for State {
    fn from(value: Vec<Vec<char>>) -> Self {
        let max_width = value[0].len().try_into().unwrap();
        let max_height = value.len().try_into().unwrap();
        let mut nodes = BTreeMap::new();
        for (y, row) in value.into_iter().enumerate() {
            for (x, col) in row.into_iter().enumerate() {
                if col != '.' {
                    nodes.entry(col).or_insert(Vec::new()).push(ICoordinate {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                }
            }
        }
        Self {
            max_height,
            max_width,
            nodes,
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: State = read_lines_of_chars(input_file).into();

    let answer_1 = input
        .all_antinodes(true, false)
        .values()
        .flatten()
        .unique()
        .count();
    let answer_2 = input
        .all_antinodes(false, true)
        .values()
        .flatten()
        .unique()
        .count();

    (answer_1, answer_2)
}

base_aoc!(14, 34);
