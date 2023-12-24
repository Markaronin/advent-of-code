use std::collections::{BTreeMap, BTreeSet};

use advent_of_code_util::{base_aoc, parse::read_lines_of_chars, Coordinate};
use itertools::Itertools;
use petgraph::{
    algo::{all_simple_paths, is_cyclic_directed},
    stable_graph::NodeIndex,
    Graph,
};

// This assumes that from and to are non diagonally adjacent and are movable
fn is_slope_accessible(from: Coordinate, to: Coordinate, input: &[Vec<char>]) -> bool {
    // ...
    // .FT
    // ...
    if from.x == to.x - 1 && input[to.y][to.x] == '<' {
        return false;
    }
    // ...
    // TF.
    // ...
    if from.x == to.x + 1 && input[to.y][to.x] == '>' {
        return false;
    }
    // ...
    // .F.
    // .T.
    if from.y == to.y - 1 && input[to.y][to.x] == '^' {
        return false;
    }
    // .T.
    // .F.
    // ...
    if from.y == to.y + 1 && input[to.y][to.x] == 'v' {
        return false;
    }
    true
}

fn get_graph(
    input: &[Vec<char>],
    include_slopes: bool,
) -> (Graph<Coordinate, usize>, NodeIndex, NodeIndex) {
    // Create new node whenever there are >1 options from a specified point
    // Otherwise, add 1 to weight
    let max_width = input[0].len();
    let max_height = input.len();

    let mut node_map: BTreeMap<Coordinate, NodeIndex> = BTreeMap::new();
    let mut graph = Graph::new();

    for y in 0..max_height {
        for x in 0..max_width {
            if input[y][x] != '#' {
                let idx = graph.add_node(Coordinate { x, y });
                node_map.insert(Coordinate { x, y }, idx);
            }
        }
    }

    let mut seen: BTreeSet<Coordinate> = BTreeSet::new();
    let mut queue: Vec<Coordinate> = Vec::new();

    queue.push(Coordinate { x: 1, y: 1 });

    while let Some(next) = queue.pop() {
        seen.insert(next);
        let mut neighbors = next
            .get_surrounding_non_diagonal_coordinates(max_width, max_height)
            .into_iter()
            .filter(|coord| input[coord.y][coord.x] != '#')
            .collect_vec();
        if include_slopes {
            neighbors.retain(|n| is_slope_accessible(next, *n, input));
        }
        for neighbor in neighbors.iter() {
            graph.add_edge(
                *node_map.get(&next).unwrap(),
                *node_map.get(neighbor).unwrap(),
                1,
            );
        }
        neighbors.retain(|n| !seen.contains(n));
        for neighbor in neighbors {
            queue.push(neighbor);
        }
    }

    (
        graph,
        *node_map
            .get(&Coordinate { x: 1, y: 1 })
            .expect("Should have found the start location"),
        *node_map
            .get(&Coordinate {
                x: max_width - 2,
                y: max_height - 2,
            })
            .expect("Should have found the end location"),
    )
}

fn calculate_longest_path(
    graph: &Graph<Coordinate, usize>,
    start: &NodeIndex,
    end: &NodeIndex,
) -> usize {
    dbg!(is_cyclic_directed(graph));
    all_simple_paths::<Vec<_>, _>(graph, *start, *end, 0, None)
        .map(|p| {
            // dbg!(&p);
            p.windows(2)
                .map(|n| {
                    graph
                        .edge_weight(graph.find_edge(n[0], n[1]).unwrap())
                        .unwrap()
                })
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines_of_chars(input_file);

    let result_1 = {
        let (graph, start, end) = get_graph(&input, true);
        calculate_longest_path(&graph, &start, &end)
    };

    let result_2 = {
        let (graph, start, end) = get_graph(&input, false);
        calculate_longest_path(&graph, &start, &end)
    };

    (result_1 + 2, result_2 + 2)
}

base_aoc!(94, 154);
