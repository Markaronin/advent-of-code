use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet, VecDeque},
};

use advent_of_code_util::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Valve {
    flow_rate: usize,
    connections: Vec<String>,
}

type GraphType = BTreeMap<String, Valve>;
type DistanceMatrixType = BTreeMap<(String, String), usize>;

fn create_distance_matrix(graph: &GraphType) -> DistanceMatrixType {
    let mut distance_matrix = DistanceMatrixType::new();

    for starting_valve_name in graph.keys() {
        let mut queue = VecDeque::new();
        let mut seen_valves = BTreeSet::new();
        seen_valves.insert(starting_valve_name.clone());
        seen_valves.extend(graph.get(starting_valve_name).unwrap().connections.clone());

        queue.extend(
            graph
                .get(starting_valve_name)
                .unwrap()
                .connections
                .clone()
                .into_iter()
                .map(|n| (n, 1)),
        );

        while let Some((valve_name, distance)) = queue.pop_front() {
            // seen_valves.insert();
            let valve = graph.get(&valve_name).unwrap();

            distance_matrix.insert((starting_valve_name.clone(), valve_name), distance);

            for connection in &valve.connections {
                if !seen_valves.contains(connection) {
                    seen_valves.insert(connection.clone());
                    queue.push_back((connection.clone(), distance + 1));
                }
            }
        }
    }

    distance_matrix
}

fn remove_zero_valves(graph: &mut GraphType, distance_matrix: &mut DistanceMatrixType) {
    let zero_valves = graph
        .iter()
        .filter(|(name, valve)| valve.flow_rate == 0 && *name != "AA")
        .map(|(name, _)| name.clone())
        .collect_vec();

    zero_valves.into_iter().for_each(|zero_valve| {
        graph.remove(&zero_valve);
        let useless_distances = distance_matrix
            .keys()
            .filter(|(first, second)| *first == zero_valve || *second == zero_valve)
            .cloned()
            .collect_vec();

        useless_distances.into_iter().for_each(|d| {
            distance_matrix.remove(&d);
        })
    });
}

fn get_best_route(graph: &GraphType, distance_matrix: &DistanceMatrixType) -> usize {
    // Input: current position, time, unopened valves, output so far

    let mut queue = VecDeque::new();
    queue.push_back((
        "AA".to_string(),
        0,
        graph
            .keys()
            .filter(|key| *key != "AA")
            .cloned()
            .collect_vec(),
        0,
    ));

    let mut best_so_far = 0;

    while let Some((valve_name, time_so_far, remaining_valves, pressure_so_far)) = queue.pop_front()
    {
        best_so_far = max(best_so_far, pressure_so_far);

        for next_valve_name in remaining_valves.clone() {
            let new_time_so_far = time_so_far
                + 1
                + distance_matrix
                    .get(&(valve_name.clone(), next_valve_name.clone()))
                    .unwrap();
            if new_time_so_far <= 30 {
                let target_valve = graph.get(&next_valve_name).unwrap();
                let new_item = (
                    next_valve_name.clone(),
                    new_time_so_far,
                    remaining_valves
                        .iter()
                        .filter(|v| **v != next_valve_name)
                        .cloned()
                        .collect_vec(),
                    pressure_so_far + ((30 - new_time_so_far) * target_valve.flow_rate),
                );
                queue.push_back(new_item);
            }
        }
    }

    best_so_far
}

fn best_path_3(mut graph: GraphType) -> usize {
    let mut distance_matrix = create_distance_matrix(&graph);

    remove_zero_valves(&mut graph, &mut distance_matrix);

    let best_route_value = get_best_route(&graph, &distance_matrix);

    best_route_value
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = {
        let raw_input = read_lines(input_file);
        let mut input: GraphType = BTreeMap::new();

        let regex = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valve?s? ([A-Z, ]+)$",
        )
        .unwrap();

        for line in raw_input {
            let captures = regex.captures(&line).unwrap();
            let valve_name = captures[1].to_string();
            let flow_rate = captures[2].parse::<usize>().unwrap();
            let connections = captures[3]
                .split(", ")
                .map(|conn| conn.to_string())
                .collect_vec();

            input.insert(
                valve_name,
                Valve {
                    flow_rate,
                    connections,
                },
            );
        }

        input
    };

    let result_1 = best_path_3(input);

    (result_1, 0)
}

base_aoc!(1651, 0);
