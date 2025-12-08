use std::{cmp::Reverse, collections::BinaryHeap};

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn size(&self, i: usize) -> usize {
        self.size[self.find(i)]
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            if self.size(i) < self.size(j) {
                self.parents[i] = j;
                self.size[j] += self.size[i];
            } else {
                self.parents[j] = i;
                self.size[i] += self.size[j];
            }
        }
    }

    pub fn find(&self, mut i: usize) -> usize {
        while self.parents[i] != i {
            i = self.parents[i];
        }
        i
    }
}

fn get_program_output(coords_file: &str) -> (usize, usize) {
    let coords = read_lines(coords_file)
        .into_iter()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect_vec();

    // Use some form of sorted list to find the top N closest connections
    let mut connections = BinaryHeap::with_capacity(coords.len() * coords.len() / 2);
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let dist = coords[i].0.abs_diff(coords[j].0).pow(2)
                + coords[i].1.abs_diff(coords[j].1).pow(2)
                + coords[i].2.abs_diff(coords[j].2).pow(2);
            connections.push(Reverse((dist, i, j)));
        }
    }

    // Loop through connections, updating both sets to actually be the same set
    let mut uf = UnionFind {
        parents: (0..coords.len()).collect_vec(),
        size: vec![1; coords.len()],
    };
    let num_connections = if cfg!(test) { 10 } else { 1000 };
    let mut current_connection = 1;
    let mut part_1 = None;
    let mut part_2 = None;

    while uf.size(0) < coords.len() {
        let (_, first, second) = connections.pop().unwrap().0;

        uf.union(first, second);

        if current_connection == num_connections {
            part_1 = Some(
                (0..coords.len())
                    .map(|i| uf.find(i))
                    .unique()
                    .map(|i| uf.size(i))
                    .sorted()
                    .rev()
                    .take(3)
                    .product(),
            );
        }

        if uf.size(0) == coords.len() {
            part_2 = Some(coords[first].0 * coords[second].0);
        }

        current_connection += 1;
    }

    (part_1.unwrap(), part_2.unwrap())
}

base_aoc!(40, 25272);
