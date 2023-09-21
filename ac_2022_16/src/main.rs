use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet, VecDeque},
};

use advent_of_code_util::*;
use itertools::Itertools;
use regex::Regex;

const MINUTES: usize = 30;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Valve {
    flow_rate: usize,
    connections: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MapState {
    pressure_so_far: usize,
    position: String,
    minute: usize,
    already_open: BTreeSet<String>,
}

fn best_path_2(graph: &BTreeMap<String, Valve>) -> usize {
    let mut map_state_queue = VecDeque::new();

    map_state_queue.push_back(MapState {
        pressure_so_far: 0,
        position: "AA".to_string(),
        minute: 1,
        already_open: BTreeSet::new(),
    });

    let mut best_so_far = 0;

    let mut cache: BTreeMap<(String, usize, BTreeSet<String>), usize> = BTreeMap::new();

    while let Some(map_state) = map_state_queue.pop_front() {
        best_so_far = max(map_state.pressure_so_far, best_so_far);

        if map_state.minute < 30 && map_state.already_open.len() < graph.len() {
            let mut additions = vec![];

            if !map_state.already_open.contains(&map_state.position) {
                let mut new_already_open = map_state.already_open.clone();
                new_already_open.insert(map_state.position.clone());
                additions.push(MapState {
                    pressure_so_far: map_state.pressure_so_far
                        + (graph.get(&map_state.position).unwrap().flow_rate
                            * (MINUTES - map_state.minute)),
                    position: map_state.position.clone(),
                    minute: map_state.minute + 1,
                    already_open: new_already_open,
                })
            }

            for connection in &graph.get(&map_state.position).unwrap().connections {
                additions.push(MapState {
                    pressure_so_far: map_state.pressure_so_far,
                    position: connection.clone(),
                    minute: map_state.minute + 1,
                    already_open: map_state.already_open.clone(),
                })
            }

            for addition in additions {
                let already_in_cache = cache
                    .get(&(
                        addition.position.clone(),
                        addition.minute,
                        addition.already_open.clone(),
                    ))
                    .map(|pressure_so_far| addition.pressure_so_far <= *pressure_so_far)
                    .unwrap_or(false);

                if !already_in_cache {
                    cache.insert(
                        (
                            addition.position.clone(),
                            addition.minute,
                            addition.already_open.clone(),
                        ),
                        addition.pressure_so_far,
                    );
                    map_state_queue.push_back(addition);
                } else {
                    println!("Cache hit!");
                }
            }

            println!("So far: {} in queue", map_state_queue.len())
        }
    }

    best_so_far
}

fn best_path(
    graph: &BTreeMap<String, Valve>,
    position: &str,
    mut time_left: usize,
    mut already_open: BTreeSet<String>,
) -> usize {
    if graph.len() == already_open.len() || time_left == 0 {
        return 0;
    }
    let to_move_to = graph.get(position).unwrap().connections[0].clone();

    time_left -= 1;
    println!("You move to valve {to_move_to}");

    let valve_steam_released = if !already_open.contains(&to_move_to) {
        time_left -= 1;
        println!("You open valve {to_move_to}");
        already_open.insert(to_move_to.clone());
        graph.get(&to_move_to).unwrap().flow_rate * time_left
    } else {
        0
    };

    let other_steam_released = best_path(graph, &to_move_to, time_left, already_open);

    valve_steam_released + other_steam_released
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = {
        let raw_input = read_lines(input_file);
        let mut input: BTreeMap<String, Valve> = BTreeMap::new();

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

    // let starting_position = "AA";

    // let result_1 = best_path(&input, starting_position, MINUTES, BTreeSet::new());
    let result_1 = best_path_2(&input);

    (result_1, 0)
}

base_aoc!(1651, 0);
