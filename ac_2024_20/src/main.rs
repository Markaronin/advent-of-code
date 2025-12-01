use std::collections::{BTreeMap, BTreeSet};

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};

const AMOUNT_SAVED_THRESHOLD: usize = if cfg!(test) { 20 } else { 100 };

fn get_nearby_path_coords(
    c: Coordinate,
    all_path_coords: &BTreeSet<Coordinate>,
    spaces_away: usize,
) -> Vec<Coordinate> {
    unimplemented!()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let mut all_path_coords = BTreeSet::new();
    let (start_coord, end_coord) = {
        let mut start = None;
        let mut end = None;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                let coord = Coordinate { x, y };
                match input[y][x] {
                    'S' => {
                        start = Some(coord);
                        all_path_coords.insert(coord);
                    }
                    'E' => {
                        end = Some(coord);
                        all_path_coords.insert(coord);
                    }
                    '.' => {
                        all_path_coords.insert(coord);
                    }
                    '#' => {}
                    _ => unreachable!(),
                }
            }
        }
        (start.unwrap(), end.unwrap())
    };

    // Get vec of every point along the path
    let path_vec: Vec<Coordinate> = todo!();
    // Create map of every point -> time left
    let space_left_for_point: BTreeMap<Coordinate, usize> = todo!();

    for c in path_vec {
        // Create function that, for a given point along the path, finds all other points within N spaces
        // Loop through every point along the path. For each, get all points within cheat range. Subtract space left from current from space left at the end. Add the manhatten distance of the cheat to get total time saved.
        todo!()
    }

    // Filter time saved > AMOUNT_SAVED_THRESHOLD

    (0, 0)
}

base_aoc!(5, 0);
