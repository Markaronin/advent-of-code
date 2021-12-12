use std::collections::HashMap;

use advent_of_code_util::*;
use itertools::Itertools;

struct Cave {
    is_small: bool,
    connections: Vec<String>,
}

fn get_all_paths_recursive_part_2(
    caves: &HashMap<String, Cave>,
    path_so_far: Vec<String>,
) -> Vec<Vec<String>> {
    let current_cave = path_so_far.last().unwrap();
    if current_cave == "end" {
        vec![path_so_far]
    } else {
        caves
            .get(current_cave)
            .unwrap()
            .connections
            .iter()
            .filter(|connection| {
                let connection_cave = caves.get(*connection).unwrap();
                !connection_cave.is_small
                    || !path_so_far.contains(*connection)
                    || (*connection != "start"
                        && *connection != "end"
                        && path_so_far
                            .iter()
                            .filter(|path_cave| path_cave == connection)
                            .count()
                            == 1
                        && caves
                            .iter()
                            .filter(|(_, possibly_small_cave)| possibly_small_cave.is_small)
                            .map(|(small_cave_name, _)| {
                                path_so_far
                                    .iter()
                                    .filter(|path_cave| *path_cave == small_cave_name)
                                    .count()
                            })
                            .max()
                            .unwrap_or(0)
                            < 2)
            })
            .flat_map(|connection| {
                get_all_paths_recursive_part_2(caves, {
                    let mut new_paths_so_far = path_so_far.clone();
                    new_paths_so_far.push(connection.clone());
                    new_paths_so_far
                })
            })
            .collect::<Vec<Vec<String>>>()
    }
}

fn get_all_paths_part_2(caves: &HashMap<String, Cave>) -> Vec<Vec<String>> {
    get_all_paths_recursive_part_2(caves, vec!["start".to_string()])
}

fn get_all_paths_recursive_part_1(
    caves: &HashMap<String, Cave>,
    path_so_far: Vec<String>,
) -> Vec<Vec<String>> {
    let current_cave = path_so_far.last().unwrap();
    if current_cave == "end" {
        vec![path_so_far]
    } else {
        caves
            .get(current_cave)
            .unwrap()
            .connections
            .iter()
            .filter(|connection| {
                let connection_cave = caves.get(*connection).unwrap();
                !connection_cave.is_small || !path_so_far.contains(*connection)
            })
            .flat_map(|connection| {
                get_all_paths_recursive_part_1(caves, {
                    let mut new_paths_so_far = path_so_far.clone();
                    new_paths_so_far.push(connection.clone());
                    new_paths_so_far
                })
            })
            .collect::<Vec<Vec<String>>>()
    }
}

fn get_all_paths_part_1(caves: &HashMap<String, Cave>) -> Vec<Vec<String>> {
    get_all_paths_recursive_part_1(caves, vec!["start".to_string()])
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let mut caves: HashMap<String, Cave> = HashMap::new();
    for line in input {
        let (start, end) = line.split('-').collect_tuple::<(&str, &str)>().unwrap();
        if !caves.contains_key(start) {
            let is_small = start == start.to_lowercase();
            caves.insert(
                start.to_string(),
                Cave {
                    is_small,
                    connections: vec![end.to_string()],
                },
            );
        } else {
            caves
                .get_mut(start)
                .unwrap()
                .connections
                .push(end.to_string());
        }
        if !caves.contains_key(end) {
            let is_small = end == end.to_lowercase();
            caves.insert(
                end.to_string(),
                Cave {
                    is_small,
                    connections: vec![start.to_string()],
                },
            );
        } else {
            caves
                .get_mut(end)
                .unwrap()
                .connections
                .push(start.to_string());
        }
    }

    let all_paths_part_1 = get_all_paths_part_1(&caves);
    let all_paths_part_2 = get_all_paths_part_2(&caves);

    (all_paths_part_1.len(), all_paths_part_2.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
        let (part_1_output, part_2_output) = get_program_output(&file_path);
        assert_eq!(part_1_output, 226);
        assert_eq!(part_2_output, 3509);
    }
}

fn main() {
    let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let (part_1_output, part_2_output) = get_program_output(&file_path);
    println!("Part 1 output: {}", part_1_output);
    println!("Part 2 output: {}", part_2_output);
}
