use std::collections::BTreeSet;

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;

fn num_unique_spaces_visited(
    grid: &[Vec<char>],
    starting_position: Coordinate,
    num_steps: usize,
) -> usize {
    let mut spaces_cache: BTreeSet<(Coordinate, usize)> = BTreeSet::new();
    let mut visit_queue = vec![(starting_position, num_steps)];

    while let Some((pos, steps_left)) = visit_queue.pop() {
        if spaces_cache.contains(&(pos, steps_left)) {
            continue;
        }
        if steps_left > 0 {
            let neighbor_spaces = pos
                .get_surrounding_non_diagonal_coordinates(grid[0].len(), grid.len())
                .into_iter()
                .filter(|c| grid[c.y][c.x] != '#')
                .map(|c| (c, steps_left - 1))
                .collect_vec();
            visit_queue.extend(neighbor_spaces);
        }
        spaces_cache.insert((pos, steps_left));
    }

    spaces_cache
        .into_iter()
        .filter(|(_, num_steps)| *num_steps == 0)
        .count()
}

fn num_unique_spaces_visited_in_infinite_grid(
    grid: &[Vec<char>],
    starting_position: Coordinate,
    num_steps: usize,
) -> usize {
    // I feel like the solution is something to do with caching entire portions of the grid
    // One downside of that is that a grid can't affect neighbors. Which seems obviously false.
    // The key is noticing that this is growing in a diamond shape and grid sections look identical along the sides
    let mut spaces_cache: BTreeSet<(Coordinate, usize)> = BTreeSet::new();
    let mut visit_queue = vec![(starting_position, num_steps)];

    while let Some((pos, steps_left)) = visit_queue.pop() {
        if spaces_cache.contains(&(pos, steps_left)) {
            continue;
        }
        if steps_left > 0 {
            let neighbor_spaces = pos
                .get_surrounding_non_diagonal_coordinates(grid[0].len(), grid.len())
                .into_iter()
                .filter(|c| grid[c.y][c.x] != '#')
                .map(|c| (c, steps_left - 1))
                .collect_vec();
            visit_queue.extend(neighbor_spaces);
        }
        spaces_cache.insert((pos, steps_left));
    }

    spaces_cache
        .into_iter()
        .filter(|(_, num_steps)| *num_steps == 0)
        .count()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_lines_of_chars(input_file);
    let starting_position = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, space)| {
                if *space == 'S' {
                    Some(Coordinate { x, y })
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap();

    input[starting_position.y][starting_position.x] = '.';

    let num_steps = if cfg!(test) { 6 } else { 64 };

    let result_1 = num_unique_spaces_visited(&input, starting_position, num_steps);

    let num_steps = if cfg!(test) { 5000 } else { 26501365 };
    let result_2 = num_unique_spaces_visited_in_infinite_grid(&input, starting_position, num_steps);

    (result_1, result_2)
}

base_aoc!(16, 16733044);
