#![allow(clippy::ptr_arg)]

use advent_of_code_util::{base_aoc, matrix::clone_column, parse::read_blocks};
use itertools::Itertools;

fn is_horizontal_reflection(block: &Vec<Vec<char>>, y: usize) -> bool {
    let height = block.len();
    let num_reflectable_units = (y + 1).min(height - (y + 1));
    for y_offset in 0..num_reflectable_units {
        if block[y - y_offset] != block[y + y_offset + 1] {
            return false;
        }
    }
    true
}
fn is_vertical_reflection(block: &[Vec<char>], x: usize) -> bool {
    let width = block[0].len();
    let num_reflectable_units = (x + 1).min(width - (x + 1));
    for x_offset in 0..num_reflectable_units {
        if clone_column(block, x - x_offset) != clone_column(block, x + x_offset + 1) {
            return false;
        }
    }
    true
}
fn find_reflection_points(block: &Vec<Vec<char>>) -> Vec<usize> {
    let mut reflection_points = vec![];
    for y in 0..block.len() - 1 {
        if is_horizontal_reflection(block, y) {
            reflection_points.push((y + 1) * 100);
        }
    }
    for x in 0..block[0].len() - 1 {
        if is_vertical_reflection(block, x) {
            reflection_points.push(x + 1);
        }
    }
    reflection_points
}

fn find_reflection_points_with_smudge(block: &Vec<Vec<char>>) -> usize {
    let original_reflection_points = *find_reflection_points(block).first().unwrap();
    for x in 0..block[0].len() {
        for y in 0..block.len() {
            let mut new_block = block.clone();
            new_block[y][x] = match new_block[y][x] {
                '#' => '.',
                '.' => '#',
                _ => panic!("Invalid block char"),
            };
            if let Some(points) = find_reflection_points(&new_block)
                .into_iter()
                .find(|points| *points != original_reflection_points)
            {
                if points != original_reflection_points {
                    return points;
                }
            }
        }
    }
    panic!("No smudge points found");
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_blocks(input_file)
        .into_iter()
        .map(|block| {
            block
                .into_iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let result_1 = input
        .iter()
        .map(|block| *find_reflection_points(block).first().unwrap())
        .sum::<usize>();

    let result_2 = input
        .iter()
        .map(find_reflection_points_with_smudge)
        .sum::<usize>();

    (result_1, result_2)
}

base_aoc!(405, 400);
