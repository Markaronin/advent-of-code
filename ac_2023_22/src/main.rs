use std::{collections::BTreeSet, str::FromStr};

use advent_of_code_util::{base_aoc, parse::read_parsed_lines};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord3 {
    x: usize,
    y: usize,
    z: usize,
}
impl FromStr for Coord3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        let z = split.next().unwrap().parse::<usize>().unwrap();

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    start: Coord3,
    end: Coord3,
}
impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('~').unwrap();
        let start = Coord3::from_str(start_str).unwrap();
        let end = Coord3::from_str(end_str).unwrap();

        if start < end {
            Ok(Self { start, end })
        } else {
            Ok(Self {
                start: end,
                end: start,
            })
        }
    }
}
impl Brick {
    pub fn fall(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
    fn copy_and_fall(&self) -> Self {
        let mut copy = self.clone();
        copy.fall();
        copy
    }
    pub fn is_under(&self, other: &Self) -> bool {
        let other = other.copy_and_fall();

        fn ranges_overlap<T: PartialOrd>(range1: (T, T), range2: (T, T)) -> bool {
            let (start1, end1) = range1;
            let (start2, end2) = range2;

            // Check if range1 overlaps with range2
            !(end1 < start2 || end2 < start1)
        }

        ranges_overlap((self.start.x, self.end.x), (other.start.x, other.end.x))
            && ranges_overlap((self.start.y, self.end.y), (other.start.y, other.end.y))
            && ranges_overlap((self.start.z, self.end.z), (other.start.z, other.end.z))
    }
}

fn fall_until_stable(input: &mut [Brick]) {
    loop {
        let mut fell = false;

        for falling_brick_index in 0..input.len() {
            if input[falling_brick_index].start.z == 0 || input[falling_brick_index].end.z == 0 {
                continue;
            }

            let mut any_support = false;
            for colliding_brick_index in 0..input.len() {
                if falling_brick_index != colliding_brick_index
                    && input[colliding_brick_index].is_under(&input[falling_brick_index])
                {
                    any_support = true;
                }
            }
            if !any_support {
                fell = true;
                input[falling_brick_index].fall();
            }
        }

        if !fell {
            break;
        }
    }
}

fn get_brick_graph(input: &[Brick]) -> Vec<BTreeSet<usize>> {
    let mut graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); input.len()];

    for brick_index in 0..input.len() {
        if input[brick_index].start.z == 0 || input[brick_index].end.z == 0 {
            continue;
        }
        for other_brick_index in 0..input.len() {
            if other_brick_index != brick_index
                && input[other_brick_index].is_under(&input[brick_index])
            {
                graph[brick_index].insert(other_brick_index);
            }
        }
    }

    graph
}

fn get_num_falls_if_brick_disintegrates(graph: &[BTreeSet<usize>], index: usize) -> usize {
    let mut falls = vec![false; graph.len()];
    falls[index] = true;

    loop {
        let mut fell = false;

        for i in 0..graph.len() {
            // If the bricks under is empty, the brick must be resting on the ground, and therefore can't fall
            if falls[i] || graph[i].is_empty() {
                continue;
            }
            if graph[i]
                .iter()
                .all(|under_brick_index| falls[*under_brick_index])
            {
                falls[i] = true;
                fell = true;
            }
        }

        if !fell || !falls.contains(&false) {
            break;
        }
    }

    // subtract 1 because the disintegrated brick doesn't count
    falls.into_iter().filter(|f| *f).count() - 1
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let mut input: Vec<Brick> = read_parsed_lines(input_file);

    fall_until_stable(&mut input);

    // - Find dependencies
    let mut result_1 = 0;
    let mut result_2 = 0;
    let graph = get_brick_graph(&input);
    for removed_brick_index in 0..input.len() {
        let num_falls = get_num_falls_if_brick_disintegrates(&graph, removed_brick_index);
        if num_falls == 0 {
            result_1 += 1;
        }
        result_2 += num_falls;
    }

    (result_1, result_2)
}

base_aoc!(5, 7);
