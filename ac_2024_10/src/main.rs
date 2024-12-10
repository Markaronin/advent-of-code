use advent_of_code_util::{base_aoc, parse::read_grid_of_digits, Coordinate};
use itertools::Itertools;

fn get_trail_scores(input: &[Vec<usize>], include_all_trails: bool) -> usize {
    let mut trail_scores: Vec<Vec<Vec<Coordinate>>> =
        vec![vec![Vec::new(); input[0].len()]; input.len()];

    let mut answer = 0;
    for amt in (0..=9).rev() {
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                let coord = Coordinate { x, y };
                if amt == 9 {
                    trail_scores[y][x] = vec![coord];
                } else if input[y][x] == amt {
                    let surrounding_coords =
                        coord.get_surrounding_non_diagonal_coordinates(input[0].len(), input.len());
                    let mut connected_trailheads = surrounding_coords
                        .into_iter()
                        .filter(|c| input[c.y][c.x] == amt + 1)
                        .flat_map(|c| trail_scores[c.y][c.x].clone())
                        .collect_vec();

                    if include_all_trails {
                        connected_trailheads =
                            connected_trailheads.into_iter().unique().collect_vec();
                    }

                    trail_scores[y][x] = connected_trailheads;
                    if amt == 0 {
                        answer += trail_scores[y][x].len();
                    }
                }
            }
        }
    }

    answer
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_grid_of_digits(input_file);
    let answer_1 = get_trail_scores(&input, true);
    let answer_2 = get_trail_scores(&input, false);

    (answer_1, answer_2)
}

base_aoc!(36, 81);
