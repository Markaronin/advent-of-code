#![allow(dead_code)]
#![allow(unused_variables)]

use std::str::FromStr;

use advent_of_code_util::{base_aoc, parse::read_parsed_lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    x: isize,
    y: isize,
    z: isize,
    dx: isize,
    dy: isize,
    dz: isize,
}
impl Hailstone {
    pub fn collides_within_bound(
        &self,
        other: &Self,
        lower_bound: isize,
        upper_bound: isize,
    ) -> bool {
        true
    }
}
impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split('@')
            .flat_map(|s| s.split(','))
            .map(|s| s.trim().parse::<isize>().unwrap());
        Ok(Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
            dx: nums.next().unwrap(),
            dy: nums.next().unwrap(),
            dz: nums.next().unwrap(),
        })
    }
}

fn num_collisions(hailstones: &[Hailstone], lower_bound: isize, upper_bound: isize) -> usize {
    let mut num_collisions = 0;

    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if hailstones[i].collides_within_bound(&hailstones[j], lower_bound, upper_bound) {
                num_collisions += 1;
            }
        }
    }

    num_collisions
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input: Vec<Hailstone> = read_parsed_lines(input_file);

    let lower_bound: isize = if cfg!(test) { 7 } else { 200000000000000 };
    let upper_bound: isize = if cfg!(test) { 27 } else { 400000000000000 };

    let xy_hailstones = input
        .clone()
        .into_iter()
        .map(|h| {
            let mut new_h = h;
            new_h.z = 0;
            new_h.dz = 0;
            new_h
        })
        .collect_vec();

    let result_1 = num_collisions(&xy_hailstones, upper_bound, lower_bound);

    (result_1, 0)
}

base_aoc!(2, 0);
