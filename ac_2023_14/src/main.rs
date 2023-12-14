use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_last_empty_coords(
    platform: &[Vec<char>],
    mut coord: Coordinate,
    direction: Direction,
) -> Coordinate {
    match direction {
        Direction::North => {
            while coord.y > 0 && platform[coord.y - 1][coord.x] == '.' {
                coord.y -= 1;
            }
        }
        Direction::East => {
            while coord.x < platform[0].len() - 1 && platform[coord.y][coord.x + 1] == '.' {
                coord.x += 1;
            }
        }
        Direction::South => {
            while coord.y < platform.len() - 1 && platform[coord.y + 1][coord.x] == '.' {
                coord.y += 1;
            }
        }
        Direction::West => {
            while coord.x > 0 && platform[coord.y][coord.x - 1] == '.' {
                coord.x -= 1;
            }
        }
    }

    coord
}

fn roll(platform: &mut [Vec<char>], direction: Direction) {
    for y in match direction {
        Direction::North | Direction::East | Direction::West => (0..platform.len()).collect_vec(),
        Direction::South => (0..platform.len()).rev().collect_vec(),
    } {
        for x in match direction {
            Direction::North | Direction::South | Direction::West => {
                (0..platform[0].len()).collect_vec()
            }
            Direction::East => (0..platform[0].len()).rev().collect_vec(),
        } {
            if platform[y][x] == 'O' {
                let to_coord = find_last_empty_coords(platform, Coordinate { x, y }, direction);
                platform[y][x] = '.';
                platform[to_coord.y][to_coord.x] = 'O';
            }
        }
    }
}

fn calculate_north_load(platform: &[Vec<char>]) -> usize {
    let mut load = 0;
    for y in 0..platform.len() {
        for x in 0..platform[0].len() {
            if platform[y][x] == 'O' {
                load += platform.len() - y
            }
        }
    }
    load
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input = read_lines_of_chars(input_file);

    let mut part_1_input = input.clone();

    roll(&mut part_1_input, Direction::North);

    let result_1 = calculate_north_load(&part_1_input);

    let mut seen_results = vec![];
    while !seen_results.contains(&input) {
        seen_results.push(input.clone());
        roll(&mut input, Direction::North);
        roll(&mut input, Direction::West);
        roll(&mut input, Direction::South);
        roll(&mut input, Direction::East);
    }
    let start_cycle_index = seen_results
        .iter()
        .find_position(|r| r == &&input)
        .unwrap()
        .0;
    let cycle_length = seen_results.len() - start_cycle_index;
    let offset = seen_results.len() - cycle_length;
    let final_cycle_index = ((1_000_000_000 - start_cycle_index) % cycle_length) + offset;
    let result_2 = calculate_north_load(&seen_results[final_cycle_index]);

    // Solution: find long cycles between cycles. Create vec of all previous ones, check against that after things have settled. Find out how long between cycles,
    // then use that info to calculate which portion of the cycle it'll be on after a billion cycles

    // Use Map from vec -> cycle index?
    // Upon first match, find cycle length. Final cycle index = ((1 billion - start of cycle index) % cycle length) + offset. Maybe an off-by-one error in that math somewhere.

    (result_1, result_2)
}

base_aoc!(136, 64);
