use std::collections::{BTreeMap, BTreeSet, VecDeque};

use advent_of_code_util::{base_aoc, parse::read_lines, Coordinate};
use itertools::Itertools;

fn connections(
    nodes: &BTreeMap<Coordinate, usize>,
    position: Coordinate,
    width: usize,
    height: usize,
) -> Vec<Coordinate> {
    position
        .get_surrounding_non_diagonal_coordinates(width, height)
        .into_iter()
        .filter(|neighbor| *nodes.get(neighbor).unwrap() <= *nodes.get(&position).unwrap() + 1)
        .collect_vec()
}

fn shortest_path_from_starting_coordinates(
    nodes: &BTreeMap<Coordinate, usize>,
    starting_coordinates: Vec<Coordinate>,
    ending_coordinate: Coordinate,
    width: usize,
    height: usize,
) -> usize {
    let mut visited: BTreeSet<Coordinate> = BTreeSet::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let mut parents: BTreeMap<Coordinate, Coordinate> = BTreeMap::new();
    queue.extend(starting_coordinates.clone());
    queue.iter().for_each(|starting_coordinate| {
        visited.insert(starting_coordinate.clone());
    });

    while let Some(next) = queue.pop_front() {
        if next == ending_coordinate {
            break;
        }
        for connection in connections(&nodes, next, width, height) {
            if !visited.contains(&connection) {
                visited.insert(connection.clone());
                parents.insert(connection.clone(), next);
                queue.push_back(connection);
            }
        }
    }

    let mut shortest_path_len = 0;
    let mut current_position = ending_coordinate.clone();
    while !starting_coordinates.contains(&current_position) {
        current_position = parents.get(&current_position).unwrap().clone();
        shortest_path_len += 1;
    }
    shortest_path_len
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let width = input[0].len();
    let height = input.len();
    let mut nodes: BTreeMap<Coordinate, usize> = BTreeMap::new();
    let mut starting_position = None;
    let mut ending_position = None;

    for (y, row) in input.into_iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let height = match c {
                'S' => {
                    starting_position = Some(Coordinate { x, y });
                    'a' as usize
                }
                'E' => {
                    ending_position = Some(Coordinate { x, y });
                    'z' as usize
                }
                other_char => other_char as usize,
            };
            nodes.insert(Coordinate { x, y }, height);
        }
    }

    let starting_position = starting_position.unwrap();
    let ending_position = ending_position.unwrap();

    let result_1 = shortest_path_from_starting_coordinates(
        &nodes,
        vec![starting_position],
        ending_position.clone(),
        width,
        height,
    );

    let result_2 = shortest_path_from_starting_coordinates(
        &nodes,
        nodes
            .iter()
            .filter(|(_, height)| **height == 'a' as usize)
            .map(|(pos, _)| pos)
            .cloned()
            .collect_vec(),
        ending_position,
        width,
        height,
    );

    (result_1, result_2)
}

base_aoc!(31, 29);
