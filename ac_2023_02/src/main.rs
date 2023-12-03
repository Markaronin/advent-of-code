use std::collections::BTreeMap;

use advent_of_code_util::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CubeColor {
    Red,
    Green,
    Blue,
}
impl CubeColor {
    pub fn from_str(s: &str) -> Self {
        match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Invalid cube color {s}"),
        }
    }
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file)
        .into_iter()
        .map(|line| {
            let after_game = line.split_once(": ").unwrap().1;
            after_game
                .split("; ")
                .map(|handful| {
                    handful
                        .split(", ")
                        .map(|amount_color| {
                            let amount_color_split = amount_color.split_once(' ').unwrap();
                            (
                                amount_color_split.0.parse::<usize>().unwrap(),
                                CubeColor::from_str(amount_color_split.1),
                            )
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    let a = input
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter().all(|handful| {
                handful.iter().all(|(amt, color)| {
                    *amt <= match color {
                        CubeColor::Red => max_red_cubes,
                        CubeColor::Green => max_green_cubes,
                        CubeColor::Blue => max_blue_cubes,
                    }
                })
            })
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let b = input
        .iter()
        .map(|game| {
            let mut maxes: BTreeMap<CubeColor, usize> = BTreeMap::new();

            for handful in game {
                for (mut amt, color) in handful {
                    let max = maxes.entry(*color).or_insert(0);
                    *max = *max.max(&mut amt);
                }
            }

            maxes.get(&CubeColor::Red).unwrap_or(&0)
                * maxes.get(&CubeColor::Green).unwrap_or(&0)
                * maxes.get(&CubeColor::Blue).unwrap_or(&0)
        })
        .sum::<usize>();

    (a, b)
}

base_aoc!(8, 2286);
