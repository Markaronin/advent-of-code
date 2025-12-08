use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

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
    let mut idx_to_circuit: Vec<Option<usize>> = vec![None; coords.len()];
    let mut circuits: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut new_circuit_index = 0;
    let num_connections = if cfg!(test) { 10 } else { 1000 };
    let mut current_connection = 1;
    let mut part_1 = None;
    let mut part_2 = None;

    while circuits.len() > 1
        || new_circuit_index < 2
        || circuits.first_entry().unwrap().get().len() < coords.len()
    {
        let (_, first, second) = connections.pop().unwrap().0;

        match (idx_to_circuit[first], idx_to_circuit[second]) {
            (None, None) => {
                let circuit_index = new_circuit_index;
                new_circuit_index += 1;
                let mut circuit = BTreeSet::new();
                circuit.insert(first);
                circuit.insert(second);
                circuits.insert(circuit_index, circuit);
                idx_to_circuit[first] = Some(circuit_index);
                idx_to_circuit[second] = Some(circuit_index);
            }
            (None, Some(second_circuit)) => {
                idx_to_circuit[first] = Some(second_circuit);
                circuits.get_mut(&second_circuit).unwrap().insert(first);
            }
            (Some(first_circuit), None) => {
                idx_to_circuit[second] = Some(first_circuit);
                circuits.get_mut(&first_circuit).unwrap().insert(second);
            }
            (Some(first_circuit), Some(second_circuit)) if first_circuit == second_circuit => {
                // Do nothing
            }
            (Some(first_circuit), Some(second_circuit)) => {
                // Merge the circuits of first and second together
                let second_circuit_coords = circuits.remove(&second_circuit).unwrap();
                for second_circuit_coord in second_circuit_coords {
                    circuits
                        .get_mut(&first_circuit)
                        .unwrap()
                        .insert(second_circuit_coord);
                    idx_to_circuit[second_circuit_coord] = Some(first_circuit);
                }
                circuits.remove(&second_circuit);
            }
        }

        if current_connection == num_connections {
            part_1 = Some(
                circuits
                    .values()
                    .map(|v| v.len())
                    .sorted()
                    .rev()
                    .take(3)
                    .product(),
            );
        }

        if circuits.len() == 1
            && new_circuit_index > 1
            && circuits.first_entry().unwrap().get().len() == coords.len()
        {
            part_2 = Some(coords[first].0 * coords[second].0);
        }

        current_connection += 1;
    }

    (part_1.unwrap(), part_2.unwrap())
}

base_aoc!(40, 25272);
