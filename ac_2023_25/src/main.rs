use std::collections::{BTreeMap, BTreeSet};

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

fn parse_input(input: Vec<String>) -> (Vec<String>, BTreeMap<String, BTreeSet<String>>) {
    let mut nodes = vec![];
    let mut edges = BTreeMap::new();

    for line in input {
        let (node, raw_edges) = line
            .split(": ")
            .map(|s| s.to_string())
            .collect_tuple()
            .unwrap();
        let node_edges: BTreeSet<String> = raw_edges
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        nodes.push(node.clone());
        edges
            .entry(node.clone())
            .or_insert(BTreeSet::new())
            .append(&mut node_edges.clone());
        for edge in node_edges {
            edges
                .entry(edge.clone())
                .or_insert(BTreeSet::new())
                .insert(node.clone());
        }
    }

    (nodes, edges)
}

fn group_sizes(nodes: &Vec<String>, edges: &BTreeMap<String, BTreeSet<String>>) -> Vec<usize> {
    // Mark all nodes unvisited
    // Set next_group = 0
    // Visit each node
    // If node is the neighbor of a node with a group, then add that node to the same group
    // Otherwise, add that node to the next group and increment next_group

    // Add every node to a stack
    // First, check if node is already in a group. If so, skip
    // If not, check if neighbors are in a group, if so,
    // Maintain a group mapping and also an "added" list

    let mut next_group = 0;
    let mut group_map: BTreeMap<String, usize> = BTreeMap::new();

    for node in nodes {
        if group_map.get(node).is_none() {
            let mut queue = vec![node];
            let group = next_group;
            next_group += 1;
            while let Some(item) = queue.pop() {
                group_map.insert(item.clone(), group);
                for neighbor in edges.get(item).unwrap() {
                    dbg!(node, item, neighbor);
                    if !group_map.contains_key(neighbor) && !queue.contains(&neighbor) {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    let mut group_sizes = vec![0; next_group];
    for v in group_map.values() {
        group_sizes[*v] += 1;
    }
    group_sizes
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let (nodes, edges) = parse_input(read_lines(input_file));

    let result_1 = group_sizes(&nodes, &edges).iter().product::<usize>();

    (result_1, 0)
}

base_aoc!(54, 0);
