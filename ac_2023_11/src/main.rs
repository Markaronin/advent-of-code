use advent_of_code_util::{base_aoc, matrix::transpose2, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;

/**
 * expansion = how many resulting empty columns from one empty column (so 1 means it'd be unchanged)
 */
fn get_coordinates_with_expansion(input: &[Vec<char>], expansion: usize) -> Vec<Coordinate> {
    let expanded_rows: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if row.iter().all(|c| *c == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect_vec();
    let expanded_cols: Vec<usize> = transpose2(input.to_vec())
        .iter()
        .enumerate()
        .filter_map(|(x, col)| {
            if col.iter().all(|c| *c == '.') {
                Some(x)
            } else {
                None
            }
        })
        .collect_vec();

    let all_galaxy_coordinates: Vec<Coordinate> = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().flat_map(move |(x, col)| {
                if *col == '#' {
                    Some(Coordinate { x, y })
                } else {
                    None
                }
            })
        })
        .map(|coord| {
            let new_x = coord.x
                + (expanded_cols.iter().filter(|col| **col < coord.x).count() * (expansion - 1));
            let new_y = coord.y
                + (expanded_rows.iter().filter(|row| **row < coord.y).count() * (expansion - 1));
            Coordinate { x: new_x, y: new_y }
        })
        .collect_vec();

    all_galaxy_coordinates
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let result_1 = get_coordinates_with_expansion(&input, 2)
        .iter()
        .combinations(2)
        .map(|coords| coords[0].non_diagonal_distance(coords[1]))
        .sum::<usize>();

    let result_2 = get_coordinates_with_expansion(&input, 1_000_000)
        .iter()
        .combinations(2)
        .map(|coords| coords[0].non_diagonal_distance(coords[1]))
        .sum::<usize>();

    (result_1, result_2)
}

base_aoc!(374, 82000210);
